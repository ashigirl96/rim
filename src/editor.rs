use std::io::{self, stdout, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::{Document, Row, Terminal};
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

fn die(e: &std::io::Error) {
    Terminal::clean_screen();
    panic!("{}", e);
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    cursor_position: Position, // ドキュメントの座標
    offset: Position,          // スクロール時の画面の原点
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(&filename).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }
}

impl Editor {
    // process_keypressによって状態を変更して、refresh_screenで画面をレンダリングする
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    // 毎度画面を再レンダリングする
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clean_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            // スクロールしている場合、
            // "カーソルの表示座標" = "内部で持ってるカーソルの座標" - "オフセット"
            // という相対位置にあるもの
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    // Welcome を画面中央にレンダリングする
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = start + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    // それぞれの行をレンダリングする
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            // スクロールを考慮して、offsetを加算する
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    // キー入力を待ち、そのキーに対応した画面の出力をする
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char(ch) => {
                if ch.is_control() {
                    println!("\r{:?}\r", ch as u8);
                } else {
                    println!("{:?} ({})\r", ch as u8, ch);
                }
            }
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('d' | 'u') => self.move_vim_binding(pressed_key),
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => println!("{:?}\r", pressed_key),
        };
        self.scroll();
        Ok(())
    }

    // スクロール
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_vim_binding(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height as usize;
        match key {
            Key::Ctrl('d') => y = y.saturating_add(height / 2),
            Key::Ctrl('u') => y = y.saturating_sub(height / 2),
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    // カーソルの座標を移動させる
    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = self.row_width(&y);
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x > 0 {
                    x = x.saturating_sub(1)
                } else if y > 0 {
                    y = y.saturating_sub(1);
                    x = self.row_width(&y);
                }
            }
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                } else if y < height {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y < terminal_height {
                    0
                } else {
                    y - terminal_height
                }
            }
            Key::PageDown => {
                y = if height < y.saturating_add(terminal_height) {
                    height
                } else {
                    terminal_height + y
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        // y: キーによって更新された新たなy座標。このy座標に対応する行の長さを最長幅とする。
        // xが最長幅よりながければ、最長幅に合わせる
        width = self.row_width(&y);
        if x > width {
            x = width;
        }
        self.cursor_position = Position { x, y }
    }

    // y座標に対する行の最大幅
    fn row_width(&self, y: &usize) -> usize {
        if let Some(row) = self.document.row(*y) {
            row.len()
        } else {
            0
        }
    }
}
