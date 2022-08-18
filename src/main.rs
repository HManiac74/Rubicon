use crossterm::terminal;
use std::io;
use std::io::Read;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    let mut buf = [0; 1];
     while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf[0] != b'q' {
         let character = buf[0] as char;
         if character.is_control() {
             println!("{}", character as u8)
         } else {
             println!("{}", character)
         }
     }
}