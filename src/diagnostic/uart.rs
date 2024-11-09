use crate::common;
use crate::io::uart;

pub fn test_uart() -> ! {
  use common::stream;
  stream::println!("UART TEST");
  stream::println!("Hello, kernel World from Rust!");

  stream::println!(
    "Decimal number print test (expected: 1234567890): {}",
    1234567890
  );
  stream::println!(
    "Hexadecimal number print test (expected: 0xCAFECAFE): 0x{:X}",
    0xCAFECAFE as i64
  );

  loop {
    uart::putc(uart::getc());
  }
}
