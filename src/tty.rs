use crate::common;

pub type Result<T> = core::result::Result<T, TtyError>;

static mut DEFAULT_TTY: core::mem::MaybeUninit<&mut Tty> =
  core::mem::MaybeUninit::<&mut Tty>::uninit();

#[derive(Debug, PartialEq)]
pub enum TtyError {
  WriteError,
  ReadError,
}

const INPUT_BUFFER_SIZE: usize = 256;
const OUTPUT_BUFFER_SIZE: usize = 256;
const BACKSPACE: u8 = b'\x08';
const CARRIAGE_RETURN: u8 = b'\r';
const LINE_FEED: u8 = b'\n';
const CTRL_C: u8 = b'\x03';
const DELETE: u8 = b'\x7f';

pub struct Tty {
  stream_impl: TtyStreamAdapter,
  input_buffer: [u8; INPUT_BUFFER_SIZE],
  input_pos: usize,
  output_buffer: [u8; OUTPUT_BUFFER_SIZE],
  output_pos: usize,
  echo_enabled: bool,
}

// Adapter struct for implementations
pub struct TtyStreamAdapter {
  pub read_char: fn() -> u8,
  pub write_char: fn(u8),
}

impl Tty {
  pub fn new(stream_impl: TtyStreamAdapter) -> Self {
    Self {
      stream_impl,
      input_buffer: [b'\0'; INPUT_BUFFER_SIZE],
      input_pos: 0,
      output_buffer: [b'\0'; OUTPUT_BUFFER_SIZE],
      output_pos: 0,
      echo_enabled: true,
    }
  }

  pub fn set_echo(&mut self, enabled: bool) {
    self.echo_enabled = enabled;
  }

  pub fn write(&mut self, output: &str) {
    for ch in output.as_bytes() {
      self.write_char(*ch);
    }
    self.flush();
  }

  pub fn read_char(&mut self) -> Result<u8> {
    match (self.stream_impl.read_char)() {
      0 => Err(TtyError::ReadError),
      c => Ok(c),
    }
  }

  pub fn write_char(&mut self, ch: u8) {
    if self.output_pos >= OUTPUT_BUFFER_SIZE {
      self.flush();
    }

    self.output_buffer[self.output_pos] = ch;
    self.output_pos += 1;
  }

  pub fn flush(&mut self) {
    for &ch in &self.output_buffer[..self.output_pos] {
      (self.stream_impl.write_char)(ch as u8);
    }
    self.output_pos = 0;
  }
}

pub unsafe fn set_as_default_tty_and_stream(tty: &'static mut Tty) {
  DEFAULT_TTY = core::mem::MaybeUninit::<&mut Tty>::new(tty);
  common::stream::assign(common::stream::OutputOps {
    write: |s| {
      DEFAULT_TTY.assume_init_mut().write(s);
      core::result::Result::Ok(())
    },
  });
}

impl core::fmt::Write for Tty {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.write(s);
    Ok(())
  }
}

#[cfg(test)]
#[cfg(feature = "host")]
#[path = "tty_test.rs"]
mod tty_test;
