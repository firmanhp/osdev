// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
// https://github.com/raspberrypi/firmware/wiki/Mailboxes
// https://github.com/raspberrypi/firmware/wiki/Accessing-mailboxes

/*
Mailboxes facilitate communication between the ARM and the VideoCore.
Each mailbox is an 8-deep FIFO of 32-bit words, which can be read (popped) or
written (pushed) by the ARM and VC.
Only mailbox 0's status can trigger interrupts on the ARM,
so MB 0 is always for communication from VC to ARM
and MB 1 is for ARM to VC.

The ARM should never write MB 0 or read MB 1.
*/

pub mod tag;
use crate::io::mmio;

struct Reg {}
impl Reg {
  const BASE: u64 = 0x0000_B_000;
  const MAIL0_READ: u64 = Reg::BASE + 0x880; // Read VC -> ARM
  const MAIL0_STA: u64 = Reg::BASE + 0x898; // Mailbox status

  const MAIL1_WRITE: u64 = Reg::BASE + 0x8a0; // Write ARM -> VC
}

struct Bit {}
impl Bit {
  const CHANNEL_MASK: u32 = 0xF;
  const CHANNEL_ARM_TO_VC: u32 = 0x8;
  const MAIL_STATUS_FULL: u32 = 0x8000_0000;
  const MAIL_STATUS_EMPTY: u32 = 0x4000_0000;
}

pub struct ResponseCode {}
impl ResponseCode {
  pub const CODE_REQUEST_SUCCESS: u32 = 0x8000_0000;
  pub const CODE_PARSE_ERROR: u32 = 0x8000_0010;
}

/*
Mailbox messages:

- The mailbox interface has 28 bits (MSB) available for the value
  and 4 bits (LSB) for the channel.
  - Request message: 28 bits (MSB) buffer address
  - Response message: 28 bits (MSB) buffer address
- Channels 8 and 9 are used.
  - Channel 8: Request from ARM for response by VC
  - Channel 9: Request from VC for response by ARM (none currently defined)


Buffer contents:
  u32: buffer size in bytes (including the header values, the end tag and padding)
  u32: buffer request/response code
    Request codes:
      0x00000000: process request
      All other values reserved
    Response codes:
      0x80000000: request successful
      0x80000001: error parsing request buffer (partial response)
  All other values reserved
    u8...: sequence of concatenated tags
    u32: 0x0 (end tag)
    u8...: padding
*/

// TODO: study response address regions?

#[repr(C, align(16))]
pub struct Message<const N: usize> {
  buf_len_bytes: u32,
  // Always 0 for request
  code: u32,
  // Raw tag buffer. align to 32 bits
  pub tag_buf: [u32; N],
  // Always 0
  tag_end: u32,
}

// Allocates a message that can hold N words (4 bytes) of tag data.
pub struct MessageBuilder<const N: usize> {
  data: Message<N>,
  fill_count_bytes: usize,
}

impl<const N: usize> Message<N> {
  pub fn builder() -> MessageBuilder<N> {
    MessageBuilder::<N> {
      data: Message::<N> {
        buf_len_bytes: N as u32 + (4 * 3),
        code: 0x0000_0000,
        tag_buf: [0; N],
        tag_end: 0,
      },
      fill_count_bytes: 0,
    }
  }
}

/*
 Tag format:
  u32: tag identifier
  u32: value buffer size in bytes
  u32:
Request codes:
  b31 clear: request
  b30-b0: reserved
Response codes:
  b31 set: response
  b30-b0: value length in bytes

u8...: value buffer
u8...: padding to align the tag to 32 bits.
*/
impl<const N: usize> MessageBuilder<N> {
  // Fill the message with a tag with ID and buffer.
  pub fn add_tag(mut self, tag: &impl MessageTag) -> MessageBuilder<N> {
    let buf = tag.value_buf();
    // + (id, 4b) + (code, 4b) + (value_buffer_size, 4b)
    let size: usize = buf.len() + 3;
    if self.fill_count_bytes + size > N {
      panic!("Message full");
    }

    self.data.tag_buf[self.fill_count_bytes] = tag.id();
    self.data.tag_buf[self.fill_count_bytes + 1] = buf.len() as u32;
    self.data.tag_buf[self.fill_count_bytes + 2] = 0x0000_0000;
    self.fill_count_bytes += 3;
    self.data.tag_buf[self.fill_count_bytes..(self.fill_count_bytes + buf.len())]
      .clone_from_slice(buf);
    self
  }

  pub fn build(self) -> Message<N> {
    self.data
  }
}

// Message tag buffer must be 4 byte aligned.
// FIXME: could have better design
pub trait MessageTag
where
  Self: Sized,
{
  // FIXME: this shouldn't be a function
  // Returns the ID of the tag.
  fn id(&self) -> u32;
  // Value buffer of the tag.
  fn value_buf(&self) -> &[u32] {
    unsafe {
      ::core::slice::from_raw_parts(
        (self as *const Self) as *const u32,
        ::core::mem::size_of::<Self>() / 4,
      )
    }
  }
}

// we only support single threaded calls now...
pub fn send<const N: usize>(message: Message<N>) -> Message<N> {
  // https://bitbanged.com/posts/understanding-rpi/the-mailbox/
  // Wait until the read end can accommodate new mails
  let raw_buf_ptr = &message as *const Message<N>;
  // Only support channel 8 for now (ARM to GPU)
  let message = raw_buf_ptr as u32 & !Bit::CHANNEL_MASK | Bit::CHANNEL_ARM_TO_VC;
  while mmio::read(Reg::MAIL0_STA) & Bit::MAIL_STATUS_FULL != 0 {}
  mmio::write(Reg::MAIL1_WRITE, message);

  // Wait until the read end receives the mail
  while (mmio::read(Reg::MAIL0_STA) & Bit::MAIL_STATUS_EMPTY != 0)
    || (mmio::read(Reg::MAIL0_READ) != message)
  {}

  // We re-read the message written by VC.
  unsafe { ::core::ptr::read_volatile(raw_buf_ptr) }
}
