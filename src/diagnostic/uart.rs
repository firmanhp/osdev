use crate::common;
use crate::io::uart;
use crate::meta;

pub fn test_uart() -> ! {
  use common::stream;

  uart::pl011_init();
  stream::println!("UART TEST");
  stream::println!("Hello, kernel World from Rust!");

  match meta::board_info::raspi_board_type() {
    meta::board_info::RaspiBoardType::Pi3 => stream::println!("I am RPi 3"),
    meta::board_info::RaspiBoardType::Pi4 => stream::println!("I am RPi 4"),
    _ => stream::println!("I am RPi unsupported"),
  };

  stream::println!(
    "Decimal number print test (expected: 1234567890): {}",
    1234567890
  );
  stream::println!(
    "Hexadecimal number print test (expected: 0xCAFECAFE): 0x{:X}",
    0xCAFECAFE as i64
  );

  loop {
    uart::pl011_putc(uart::pl011_getc());
  }
}
