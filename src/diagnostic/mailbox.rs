use crate::common::stream;
use crate::io::clock;
use crate::io::mailbox;
use crate::io::mailbox::tag::GetClockRate;
use crate::io::mailbox::tag::GetClockState;
use crate::io::mailbox::Message;
use crate::io::mailbox::MessageView;
use crate::io::uart;

// Returns how many bytes printed
fn print_message_buf<const N: usize>(message: &Message<N>) -> usize {
  let buf: &[u32] = unsafe {
    core::slice::from_raw_parts(
      message as *const Message<N> as *const u32,
      core::mem::size_of::<Message<N>>() / 4,
    )
  };

  stream::println!("::META");
  // First 2 words = Message metadata
  for (_, val) in buf[0..2].iter().enumerate() {
    stream::println!("{:#010X}", val);
  }

  let mut idx = 2;
  while idx < buf.len() {
    let tag_id = buf[idx];
    stream::println!("::ID");
    stream::println!("{:#010X}", buf[idx]);
    idx += 1;

    if tag_id == 0 {
      break;
    }

    let mut buffer_count: i64 = buf[idx].into();
    stream::println!("::SIZE_BYTES");
    stream::println!("{:#010X}", buf[idx]);
    idx += 1;

    // Req/resp code
    stream::println!("::CODE");
    stream::println!("{:#010X}", buf[idx]);
    idx += 1;

    stream::println!("::PAYLOAD");
    while buffer_count > 0 {
      stream::println!("{:#010X}", buf[idx]);
      idx += 1;
      buffer_count -= 4;
    }
  }

  // We print by 4 bytes
  return idx as usize * 4;
}

// Uses PL011 uart
pub fn test_mailbox() -> ! {
  // Test Clock mailbox
  uart::uart_init_with_stream!(bcm2837_pl011);
  stream::println!("Mailbox test: UART Clock rate");

  let clock_rate_tag: GetClockRate::Tag = GetClockRate::Request {
    clock_id: clock::ClockId::Uart as u32,
  }
  .into();
  let clock_state_tag: GetClockState::Tag = GetClockState::Request {
    clock_id: clock::ClockId::Uart as u32,
  }
  .into();

  stream::println!(
    "GetClockRate::Tag::MESSAGE_LEN = {}",
    GetClockRate::Tag::MESSAGE_LEN
  );
  stream::println!(
    "GetClockState::Tag::MESSAGE_LEN = {}",
    GetClockState::Tag::MESSAGE_LEN
  );
  type Message = mailbox::Message<
    { GetClockRate::Tag::MESSAGE_LEN + GetClockState::Tag::MESSAGE_LEN },
  >;

  let mut message = Message::builder()
    .add_tag(&clock_rate_tag)
    .add_tag(&clock_state_tag)
    .build();

  stream::println!("=========================");
  stream::println!("Request message");
  print_message_buf(&message);

  message = mailbox::send(message);
  stream::println!("=========================");
  stream::println!("Response message");
  let bytes_printed = print_message_buf(&message);

  stream::println!(
    "Message bytes = {}, Printed bytes = {}",
    &message.size(),
    bytes_printed
  );
  if message.size() != bytes_printed {
    panic!("Printed bytes are not equal");
  }

  let address: u64 = (&message as *const Message) as u64;
  stream::println!("=========================");
  stream::println!("Test mailbox message address alignment (16 bytes).");
  stream::println!("Address: {:#X}", address);

  if address % 16 != 0 {
    panic!("Test failed");
  }

  stream::println!();

  stream::println!("=========================");
  stream::println!("Test parsing responses");
  match GetClockRate::read_response(&message) {
    Ok(response) => {
      let clock_id = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.clock_id))
      };
      let clock_rate_hz = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.rate_hz))
      };
      stream::println!(
        "GetClockRate --> ID: {}, Rate: {}Hz",
        clock_id,
        clock_rate_hz
      );
    }
    Err(kind) => {
      stream::println!(
        "Reading response for GetClockRate failed: {}",
        kind as u32
      );
    }
  }
  match GetClockState::read_response(&message) {
    Ok(response) => {
      let clock_id = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.clock_id))
      };
      let state_bits = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.state_bits))
      };
      stream::println!(
        "GetClockState --> ID: {}, State bits: {} (expected: 1)",
        clock_id,
        state_bits
      );
    }
    Err(kind) => {
      stream::println!(
        "Reading response for GetClockRate failed: {}",
        kind as u32
      );
    }
  }
  stream::println!("Test OK.");
  loop {}
}
