use std::io::{self, stdout, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::terminal::Terminal;

fn die(e: &std::io::Error) {
    Terminal::clean_screen();
    panic!("{}", e);
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
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

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        // print!("\x1b[0J");
        Terminal::clean_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

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
            _ => println!("{:?}\r", pressed_key),
        };
        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}
