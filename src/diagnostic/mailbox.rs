use crate::io::clock;
use crate::io::mailbox;
use crate::io::mailbox::MessageTag;
use crate::io::uart;

// Uses PL011 uart
pub fn test_mailbox() {
  // Test Clock mailbox
  uart::pl011_puts("Mailbox test: UART Clock rate\r\n");
  let clock_rate_tag = mailbox::tag::GetClockRate {
    clock_id: clock::ClockId::UART,
  };

  uart::pl011_puts("Test raw mailbox tag.\r\n");
  uart::pl011_puts("Expect pointer value to be: ");
  uart::pl011_putint(clock::ClockId::UART.into());
  uart::pl011_puts("\r\n");

  let value_buf: u32 = clock_rate_tag.value_buf()[0].into();
  uart::pl011_puts("Actual pointer value: ");
  uart::pl011_putint(value_buf.into());
  uart::pl011_puts("\r\n");

  if value_buf != clock::ClockId::UART {
    uart::pl011_puts("Test failed.\r\n");
    return;
  }

  // Req 4 bytes (1w)
  // Resp 8 bytes (2w)
  // + tag info (12 bytes) (3w)
  let message = mailbox::send(
    mailbox::Message::<6>::builder()
      .add_tag(&clock_rate_tag)
      .build(),
  );

  let address: u64 = (&message as *const mailbox::Message<6>) as u64;
  uart::pl011_puts("Test mailbox message address alignment (16 bytes).\r\n");
  uart::pl011_puts("Address: ");
  uart::pl011_puthex(address);
  uart::pl011_puts("\r\n");

  if address % 16 != 0 {
    uart::pl011_puts("Test failed.\r\n");
    return;
  }
}
