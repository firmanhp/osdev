use crate::io::uart::{pl011_getc, pl011_putc};
use core::{fmt, result};

pub type Result<T> = result::Result<T, TTYError>;

#[derive(Debug, PartialEq)]
pub enum TTYError {
  WriteError,
  ReadError,
  BufferOverflow,
}

const INPUT_BUFFER_SIZE: usize = 256;
const OUTPUT_BUFFER_SIZE: usize = 256;
const BACKSPACE: char = '\x08';
const CARRIAGE_RETURN: char = '\r';
const LINE_FEED: char = '\n';
const CTRL_C: char = '\x03';
const DELETE: char = '\x7f';

pub struct TTY {
  input_buffer: [char; INPUT_BUFFER_SIZE],
  input_pos: usize,
  output_buffer: [char; OUTPUT_BUFFER_SIZE],
  output_pos: usize,
  echo_enabled: bool,
}

impl TTY {
  pub fn new() -> Self {
    Self {
      input_buffer: ['\0'; INPUT_BUFFER_SIZE],
      input_pos: 0,
      output_buffer: ['\0'; OUTPUT_BUFFER_SIZE],
      output_pos: 0,
      echo_enabled: true,
    }
  }

  pub fn set_echo(&mut self, enabled: bool) {
    self.echo_enabled = enabled;
  }

  pub fn write(&mut self, output: &str) -> Result<()> {
    for ch in output.chars() {
      self.write_char(ch)?;
    }
    self.flush()?;
    Ok(())
  }

  pub fn read_line(&mut self) -> Result<String> {
    self.input_pos = 0;

    loop {
      let ch = self.read_char()?;

      match ch {
        CTRL_C => return Err(TTYError::ReadError),
        CARRIAGE_RETURN | LINE_FEED => {
          if self.echo_enabled {
            self.write_char(LINE_FEED)?;
          }
          break;
        }
        BACKSPACE | DELETE => {
          if self.input_pos > 0 {
            self.input_pos -= 1;
            if self.echo_enabled {
              self.write_str("\x08 \x08")?; // Echo backspace
            }
          }
        }
        ch if ch.is_ascii() => {
          if self.input_pos >= INPUT_BUFFER_SIZE - 1 {
            return Err(TTYError::BufferOverflow);
          }
          self.input_buffer[self.input_pos] = ch;
          self.input_pos += 1;
          if self.echo_enabled {
            self.write_char(ch)?;
          }
        }
        _ => {} // Ignore non-ASCII characters
      }
    }

    let input: String = self.input_buffer[..self.input_pos].iter().collect();
    Ok(input)
  }

  #[cfg(not(test))]
  pub fn read_char(&mut self) -> Result<char> {
    match pl011_getc() {
      0 => Err(TTYError::ReadError),
      c => Ok(c as char),
    }
  }

  #[cfg(not(test))]
  pub fn write_char(&mut self, ch: char) -> Result<()> {
    if self.output_pos >= OUTPUT_BUFFER_SIZE {
      self.flush()?;
    }

    self.output_buffer[self.output_pos] = ch;
    self.output_pos += 1;

    Ok(())
  }

  #[cfg(not(test))]
  pub fn flush(&mut self) -> Result<()> {
    for &ch in &self.output_buffer[..self.output_pos] {
      pl011_putc(ch as u8);
    }
    self.output_pos = 0;
    Ok(())
  }

  pub fn write_str(&mut self, s: &str) -> Result<()> {
    for ch in s.chars() {
      self.write_char(ch)?;
    }
    Ok(())
  }
}

impl fmt::Write for TTY {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write(s).map_err(|_| fmt::Error)
  }
}

impl Default for TTY {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
#[path = "tty_test.rs"]
mod tty_test;
