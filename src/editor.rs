use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn die(e: std::io::Error) {
    panic!("{}", e);
}

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(ch) => {
                        if ch.is_control() {
                            println!("\r{:?}\r", ch as u8);
                        } else {
                            println!("{:?} ({})\r", ch as u8, ch);
                        }
                    }
                    Key::Ctrl('q') => break,
                    // Key::Backspace => {}
                    // Key::Left => {}
                    // Key::Right => {}
                    // Key::Up => {}
                    // Key::Down => {}
                    // Key::Home => {}
                    // Key::End => {}
                    // Key::PageUp => {}
                    // Key::PageDown => {}
                    // Key::BackTab => {}
                    // Key::Delete => {}
                    // Key::Insert => {}
                    // Key::F(_) => {}
                    // Key::Alt(_) => {}
                    // Key::Null => {}
                    // Key::Esc => {}
                    // Key::__IsNotComplete => {}
                    _ => println!("{:?}\r", key),
                },
                Err(err) => die(err),
            }
        }
    }
}
