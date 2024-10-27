use crate::common::error::ErrorKind;
use crate::io::mailbox::tag::MessageTag;
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

#[repr(C, align(16))]
pub struct Message<const N: usize> {
  buf_len_bytes: u32,
  // Always 0 for request
  code: u32,
  // Raw tag buffer. align to 32 bits
  tag_buf: [u32; N],
  // Always 0
  tag_end: u32,
}

// Provides a convenient view for a generic length Message.
pub trait MessageView {
  fn size(&self) -> usize;
  fn code(&self) -> u32;
  fn tag_buffer_lookup(&self, tag_id: u32) -> Result<&[u8], ErrorKind>;
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
        buf_len_bytes: 4 * (3 + N as u32),
        code: 0x0000_0000,
        tag_buf: [0; N],
        tag_end: 0,
      },
      fill_count_bytes: 0,
    }
  }
}

impl<const N: usize> MessageView for Message<N> {
  fn size(&self) -> usize {
    return self.buf_len_bytes as usize;
  }
  fn code(&self) -> u32 {
    return self.code;
  }
  fn tag_buffer_lookup(&self, tag_id: u32) -> Result<&[u8], ErrorKind> {
    let len_u8 = self.tag_buf.len() * 4;
    let tag_buf_u8 = unsafe {
      core::slice::from_raw_parts(
        self.tag_buf.as_ptr() as *const u8,
        len_u8 as usize,
      )
    };

    let mut idx_u8 = 0;
    while idx_u8 < len_u8 {
      // First 12 bytes are metadata.
      if idx_u8 + 12 > len_u8 {
        panic!("Invalid message structure!");
      }
      let meta_u32 = unsafe {
        core::slice::from_raw_parts(
          tag_buf_u8[idx_u8..].as_ptr() as *const u32,
          3,
        )
      };
      let id = meta_u32[0];
      let tag_size_bytes = meta_u32[1] as usize;
      // meta_u32[2] = req/resp. code.
      let next = idx_u8 + 12 + tag_size_bytes;

      if id == tag_id {
        if next > len_u8 {
          panic!("Tag size exceeds message buffer");
        }
        return Ok(&tag_buf_u8[idx_u8..next]);
      }

      if id == 0 {
        // We reached end of tag
        break;
      }
      idx_u8 = next;
    }

    Err(ErrorKind::NotFound)
  }
}

impl<const N: usize> MessageBuilder<N> {
  // Fill the message with a tag with ID and buffer.
  pub fn add_tag(mut self, tag: &impl MessageTag) -> MessageBuilder<N> {
    // let buf = tag.value_buf();
    // + (id, 4b) + (value_buffer_size, 4b) + (code, 4b)
    let size: usize = tag.size_bytes();
    if self.fill_count_bytes + size > N * 4 {
      panic!("Message full");
    }

    // Make u8 view of data.tag_buf here.
    let buf: &mut [u8] = unsafe {
      core::slice::from_raw_parts_mut(
        self.data.tag_buf.as_mut_ptr() as *mut u8,
        N * 4,
      )
    };
    let tag_buf: &[u8] = tag.buf();

    // First 3 words (4b) are metadata
    buf[self.fill_count_bytes..(self.fill_count_bytes + 12)]
      .clone_from_slice(&tag_buf[0..12]);
    self.fill_count_bytes += 12;

    buf[self.fill_count_bytes
      ..(self.fill_count_bytes + tag.payload_size_bytes())]
      .clone_from_slice(&tag_buf[12..(12 + tag.payload_size_bytes())]);
    self.fill_count_bytes += tag.payload_size_bytes();

    self
  }

  pub fn build(self) -> Message<N> {
    self.data
  }
}

// we only support single threaded calls now...
pub fn send<const N: usize>(message: Message<N>) -> Message<N> {
  // https://bitbanged.com/posts/understanding-rpi/the-mailbox/
  // Wait until the read end can accommodate new mails
  let raw_buf_ptr = &message as *const Message<N>;
  // Only support channel 8 for now (ARM to GPU)
  let message =
    raw_buf_ptr as u32 & !Bit::CHANNEL_MASK | Bit::CHANNEL_ARM_TO_VC;
  while mmio::read(Reg::MAIL0_STA) & Bit::MAIL_STATUS_FULL != 0 {}
  mmio::write(Reg::MAIL1_WRITE, message);

  // Wait until the read end receives the mail
  while (mmio::read(Reg::MAIL0_STA) & Bit::MAIL_STATUS_EMPTY != 0)
    || (mmio::read(Reg::MAIL0_READ) != message)
  {}

  // We re-read the message written by VC.
  unsafe { ::core::ptr::read_volatile(raw_buf_ptr) }
}
