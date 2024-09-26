use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(val) => {
                let ch = val as char;
                if ch.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", val);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:?}\r", val, ch);
                }
                if ch == 'q' {
                    break;
                }
            }
            Err(err) => println!("Error: {}", err)
        }
    }
    disable_raw_mode().unwrap();
}
