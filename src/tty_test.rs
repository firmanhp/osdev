use super::TTY;

// Mock UART functions for testing
static mut MOCK_UART_BUFFER: Vec<u8> = Vec::new();

fn mock_putc(c: u8) {
    unsafe { MOCK_UART_BUFFER.push(c); }
}

fn mock_getc() -> u8 {
    b'H'
}

// Override TTY methods for testing
impl TTY {
    pub(crate) fn write_char(&self, ch: char) {
        mock_putc(ch as u8);
    }

    pub(crate) fn read_char(&self) -> Option<char> {
        Some(mock_getc() as char)
    }
}

#[test]
fn test_tty_write() {
    let tty = TTY::new();
    unsafe { MOCK_UART_BUFFER.clear(); }
    tty.write("Hello, TTY!");

    unsafe {
        let expected_output = b"Hello, TTY!";
        assert_eq!(&MOCK_UART_BUFFER[..], expected_output);
    }
}

#[test]
fn test_tty_read() {
    let tty = TTY::new();
    if let Some(ch) = tty.read() {
        assert_eq!(ch, 'H');
    } else {
        panic!("Failed to read character from TTY");
    }
}
