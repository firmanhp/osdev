use super::{TTYError, TTY};
use core::fmt::Write;
use std::sync::Mutex;

// Mock UART buffer for testing
static MOCK_UART_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static MOCK_INPUT: Mutex<Vec<char>> = Mutex::new(Vec::new());

// Test helper functions
fn setup() {
  *MOCK_UART_BUFFER.lock().unwrap() = Vec::new();
  *MOCK_INPUT.lock().unwrap() = Vec::new();
}

fn mock_putc(c: u8) {
  MOCK_UART_BUFFER.lock().unwrap().push(c);
}

fn mock_getc() -> u8 {
  MOCK_INPUT.lock().unwrap().pop().unwrap_or('\0' as char) as u8
}

fn set_mock_input(input: &str) {
  let mut mock_input = MOCK_INPUT.lock().unwrap();
  mock_input.clear();
  mock_input.extend(input.chars().rev());
}

fn get_output() -> Vec<u8> {
  MOCK_UART_BUFFER.lock().unwrap().clone()
}

// Test implementation of TTY methods
#[cfg(test)]
impl TTY {
  pub fn write_char(&mut self, ch: char) -> Result<(), TTYError> {
    mock_putc(ch as u8);
    Ok(())
  }

  pub fn read_char(&mut self) -> Result<char, TTYError> {
    let c = mock_getc();
    if c == 0 {
      Err(TTYError::ReadError)
    } else {
      Ok(c as char)
    }
  }

  pub fn flush(&mut self) -> Result<(), TTYError> {
    Ok(())
  }
}

#[test]
fn test_tty_write() {
  setup();
  let mut tty = TTY::new();
  tty.write("Hello, TTY!").expect("Write should succeed");

  assert_eq!(get_output(), b"Hello, TTY!");
}

#[test]
fn test_tty_write_fmt() {
  setup();
  let mut tty = TTY::new();
  write!(tty, "Value: {}", 42).expect("Write formatting should succeed");

  assert_eq!(get_output(), b"Value: 42");
}

#[test]
fn test_tty_read_line() {
  setup();
  let mut tty = TTY::new();

  set_mock_input("Hello\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hello");

  assert_eq!(get_output(), b"Hello\n");
}

#[test]
fn test_tty_ctrl_c() {
  setup();
  let mut tty = TTY::new();

  set_mock_input("\x03");

  let result = tty.read_line();
  assert!(matches!(result, Err(TTYError::ReadError)));
}

#[test]
fn test_tty_empty_line() {
  setup();
  let mut tty = TTY::new();

  set_mock_input("\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "");

  assert_eq!(get_output(), b"\n");
}

#[test]
fn test_tty_multiple_backspaces() {
  setup();
  let mut tty = TTY::new();

  set_mock_input("\x08Hello\x08\x08\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hel");

  let output = get_output();
  assert!(output.ends_with(b"\n"));
}

#[test]
fn test_tty_echo_disable() {
  setup();
  let mut tty = TTY::new();
  tty.set_echo(false);

  set_mock_input("Hello\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hello");

  assert_eq!(get_output(), b"");
}
