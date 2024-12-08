use crate::common;
use crate::io::uart;

fn uart_on_receive(payload: uart::Payload) {
  for ch in payload {
    uart::putc(ch);
  }
}

pub fn test_uart_interrupt() -> ! {
  use common::stream;
  stream::println!("UART TEST");
  stream::println!("Testing feedback from interrupt");

  uart::set_receive_callback(uart_on_receive);
  uart::interrupt_enable();
  loop {}
}
