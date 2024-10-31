use crate::io::uart::{pl011_putc, pl011_getc};

pub struct TTY;

impl TTY {
    pub fn new() -> Self {
        TTY
    }

    pub fn write(&self, output: &str) {
        for ch in output.chars() {
            self.write_char(ch);
        }
    }

    pub fn read(&self) -> Option<char> {
        self.read_char()
    }

    #[cfg(not(test))]
    fn write_char(&self, ch: char) {
        pl011_putc(ch as u8);
    }
    
    #[cfg(not(test))]
    fn read_char(&self) -> Option<char> {
        Some(pl011_getc() as char)
    }
}

#[cfg(test)]
#[path = "tty_test.rs"]
mod tty_test;
