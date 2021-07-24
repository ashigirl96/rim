use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111 // 先頭3bitが0だと、ctrl-keyになる
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() { // tabとかdeleteとか. ASCII codes 0-31 and 127
                    println!("\r{:?}\r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                println!("{:#b}\r", b);
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }

    }
}
