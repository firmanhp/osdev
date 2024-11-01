use std::sync::Mutex;

// Mock UART buffer for testing
static MOCK_UART_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static MOCK_INPUT: Mutex<Vec<char>> = Mutex::new(Vec::new());

// Test helper functions
pub fn setup() {
  *MOCK_UART_BUFFER.lock().unwrap() = Vec::new();
  *MOCK_INPUT.lock().unwrap() = Vec::new();
}

pub fn mock_putc(c: u8) {
  MOCK_UART_BUFFER.lock().unwrap().push(c);
}

pub fn mock_getc() -> u8 {
  MOCK_INPUT.lock().unwrap().pop().unwrap_or('\0' as char) as u8
}

pub fn set_mock_input(input: &str) {
  let mut mock_input = MOCK_INPUT.lock().unwrap();
  mock_input.clear();
  mock_input.extend(input.chars().rev());
}

pub fn get_output() -> Vec<u8> {
  MOCK_UART_BUFFER.lock().unwrap().clone()
}
