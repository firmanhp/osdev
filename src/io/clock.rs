use crate::common::error::ErrorKind;
use crate::io::mailbox;

// Clock IDs
pub struct ClockId;
#[allow(dead_code)]
impl ClockId {
  pub const EMMC: u32 = 0x000000001;
  pub const UART: u32 = 0x000000002;
  pub const ARM: u32 = 0x000000003;
  pub const CORE: u32 = 0x000000004;
  pub const V3D: u32 = 0x000000005;
  pub const H264: u32 = 0x000000006;
  pub const ISP: u32 = 0x000000007;
  pub const SDRAM: u32 = 0x000000008;
  pub const PIXEL: u32 = 0x000000009;
  pub const PWM: u32 = 0x00000000a;
  pub const HEVC: u32 = 0x00000000b;
  pub const EMMC2: u32 = 0x00000000c;
  pub const M2MC: u32 = 0x00000000d;
  pub const PIXEL_BVB: u32 = 0x00000000e;
}

pub struct ClockInfo {
  pub id: u32,
  pub active: bool,
  pub exists: bool,
  // Base clock rate.
  pub rate_hz: u32,
}

pub fn get_clock_info(clock_id: u32) -> Result<ClockInfo, ErrorKind> {
  let id = clock_id;
  let active: bool;
  let exists: bool;
  let rate_hz: u32;
  // Get clock state
  {
    // Req 4 bytes (1w)
    // Resp 8 bytes (2w)
    // + tag info (12 bytes) (3w)
    let clock_state_tag = mailbox::tag::GetClockState { clock_id };
    let message = mailbox::send(
      mailbox::Message::<6>::builder()
        .add_tag(&clock_state_tag)
        .build(),
    );
    let len_bytes = value_len_bytes(message.tag_buf[2]);
    if len_bytes != 8 {
      return Err(ErrorKind::InvalidData);
    }
    // clock_id = message.tag_buf[3]
    let state_raw = message.tag_buf[4];
    active = state_raw & (1 << 0) != 0;
    exists = state_raw & (1 << 1) != 0;
    if !exists {
      return Err(ErrorKind::NotFound);
    }
  }
  {
    // Req 4 bytes (1w)
    // Resp 8 bytes (2w)
    // + tag info (12 bytes) (3w)
    let clock_rate_tag = mailbox::tag::GetClockRate { clock_id };
    let message = mailbox::send(
      mailbox::Message::<6>::builder()
        .add_tag(&clock_rate_tag)
        .build(),
    );
    let len_bytes = value_len_bytes(message.tag_buf[2]);
    if len_bytes != 8 {
      return Err(ErrorKind::InvalidData);
    }
    rate_hz = message.tag_buf[4];
  }

  Ok(ClockInfo {
    id,
    active,
    exists,
    rate_hz,
  })
}

#[inline(always)]
fn value_len_bytes(raw_response: u32) -> u32 {
  raw_response & !(1 << 31)
}
