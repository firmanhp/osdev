use crate::common;
use crate::io::clock;
use crate::io::mailbox;
use crate::io::mailbox::MessageTag;
use crate::io::uart;

// Uses PL011 uart
pub fn test_mailbox() -> ! {
  use common::stream;

  // Test Clock mailbox
  uart::pl011_init();
  stream::println!("Mailbox test: UART Clock rate");

  let clock_rate_tag = mailbox::tag::GetClockRate {
    clock_id: clock::ClockId::UART,
  };

  stream::println!("\n\n");
  stream::println!("Test raw mailbox tag.");
  stream::println!("Expect pointer value to be: {}", clock::ClockId::UART);

  let value_buf: u32 = clock_rate_tag.value_buf()[0].into();
  stream::println!("Actual pointer value: {}", value_buf);

  if value_buf != clock::ClockId::UART {
    panic!("Test failed");
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
  stream::println!("\n\n");
  stream::println!("Test mailbox message address alignment (16 bytes).");
  stream::println!("Address: 0x{:X}", address);

  if address % 16 != 0 {
    panic!("Test failed");
  }

  loop {}
}
