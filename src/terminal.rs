use crate::editor::Position;
use std::io::{self, stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size, // 移動できる画面のサイズ
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    // raw_modeにすることで、標準入力が出力されない
    pub fn default() -> Result<Self, io::Error> {
        let (width, height) = termion::terminal_size()?;
        // ステータスバーとメッセージバー用に2行分移動できる高さを減らしている
        let height = height.saturating_sub(2);
        Ok(Self {
            size: Size { width, height },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    // 画面消える。$ echo '\x1b[2J' してみるとわかる
    pub fn clean_screen() {
        print!("{}", termion::clear::All);
    }

    // 更新するたびに画面全体をクリアするのではなく、再描画するときに各行をクリアする
    // 画面すべてがclear::Allエスケープ（\x1b[J）であるのに対して、各行は(\x1b[2K)
    // $ echo '\x1b[2K'
    pub fn clear_current_line() {
        print!("{}\r", termion::clear::CurrentLine);
    }

    // 任意の座標にカーソルを固定できる
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    // カーソルを非表示する
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    // カーソルを表示する
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    // TODO: 調べる
    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    // 標準入力したものを逐次key入力にマッピングしてる
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    // その行が指定された色の背景色になる
    pub fn draw_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    // その行が元の背景色になる
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn draw_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
