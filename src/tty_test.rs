use super::{Tty, TtyError, TtyStreamAdapter};
use core::fmt::Write;
use std::sync::Mutex;

// Mock UART buffer for testing
static MOCK_UART_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static MOCK_INPUT: Mutex<Vec<char>> = Mutex::new(Vec::new());

// Test helper functions
fn make_mock() -> TtyStreamAdapter {
  *MOCK_UART_BUFFER.lock().unwrap() = Vec::new();
  *MOCK_INPUT.lock().unwrap() = Vec::new();
  TtyStreamAdapter {
    read_char: mock_getc,
    write_char: mock_putc,
  }
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

#[test]
fn test_tty_write() {
  let mut tty = Tty::new(make_mock());
  tty.write("Hello, Tty!").expect("Write should succeed");

  assert_eq!(get_output(), b"Hello, Tty!");
}

#[test]
fn test_tty_write_fmt() {
  let mut tty = Tty::new(make_mock());
  write!(tty, "Value: {}", 42).expect("Write formatting should succeed");

  assert_eq!(get_output(), b"Value: 42");
}

#[test]
fn test_tty_read_line() {
  let mut tty = Tty::new(make_mock());

  set_mock_input("Hello\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hello");

  assert_eq!(get_output(), b"Hello\n");
}

#[test]
fn test_tty_ctrl_c() {
  let mut tty = Tty::new(make_mock());

  set_mock_input("\x03");

  let result = tty.read_line();
  assert!(matches!(result, Err(TtyError::ReadError)));
}

#[test]
fn test_tty_empty_line() {
  let mut tty = Tty::new(make_mock());

  set_mock_input("\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "");

  assert_eq!(get_output(), b"\n");
}

#[test]
fn test_tty_multiple_backspaces() {
  let mut tty = Tty::new(make_mock());

  set_mock_input("\x08Hello\x08\x08\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hel");

  let output = get_output();
  assert!(output.ends_with(b"\n"));
}

#[test]
fn test_tty_echo_disable() {
  let mut tty = Tty::new(make_mock());
  tty.set_echo(false);

  set_mock_input("Hello\n");

  let line = tty.read_line().expect("Read line should succeed");
  assert_eq!(line, "Hello");

  assert_eq!(get_output(), b"");
}
