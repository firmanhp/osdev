use crate::common::error::ErrorKind;
use crate::io::mailbox;
use crate::io::mailbox::tag::GetClockRate;
use crate::io::mailbox::tag::GetClockState;
use crate::io::mailbox::Message;

// Clock IDs
#[repr(u32)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum ClockId {
  Emmc = 0x1,
  Uart = 0x2,
  Arm = 0x3,
  Core = 0x4,
  V3d = 0x5,
  H264 = 0x6,
  Isp = 0x7,
  Sdram = 0x8,
  Pixel = 0x9,
  Pwm = 0xa,
  Hevc = 0xb,
  Emmc2 = 0xc,
  M2mc = 0xd,
  PixelBvb = 0xe,
}

impl ClockId {
  pub fn as_str(&self) -> &str {
    match self {
      ClockId::Emmc => "Emmc",
      ClockId::Uart => "Uart",
      ClockId::Arm => "Arm",
      ClockId::Core => "Core",
      ClockId::V3d => "V3d",
      ClockId::H264 => "H264",
      ClockId::Isp => "Isp",
      ClockId::Sdram => "Sdram",
      ClockId::Pixel => "Pixel",
      ClockId::Pwm => "Pwm",
      ClockId::Hevc => "Hevc",
      ClockId::Emmc2 => "Emmc2",
      ClockId::M2mc => "M2mc",
      ClockId::PixelBvb => "PixelBvb",
    }
  }
}

impl core::fmt::Display for ClockId {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

pub struct ClockInfo {
  pub id: ClockId,
  pub active: bool,
  pub exists: bool,
  // Base clock rate.
  pub rate_hz: u32,
}

pub fn get_clock_info(clock_id: ClockId) -> Result<ClockInfo, ErrorKind> {
  let active: bool;
  let exists: bool;
  let rate_hz: u32;

  let clock_rate_tag: GetClockRate::Tag = GetClockRate::Request {
    clock_id: clock_id as u32,
  }
  .into();
  let clock_state_tag: GetClockState::Tag = GetClockState::Request {
    clock_id: clock_id as u32,
  }
  .into();

  let message = mailbox::send(
    Message::<
      { GetClockRate::Tag::MESSAGE_LEN + GetClockState::Tag::MESSAGE_LEN },
    >::builder()
    .add_tag(&clock_rate_tag)
    .add_tag(&clock_state_tag)
    .build(),
  );

  match GetClockRate::read_response(&message) {
    Ok(response) => {
      rate_hz = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.rate_hz))
      };
    }
    Err(_) => panic!("Error in retrieving GetClockRate"),
  }
  match GetClockState::read_response(&message) {
    Ok(response) => {
      let state_bits = unsafe {
        core::ptr::read_unaligned(core::ptr::addr_of!(response.state_bits))
      };
      active = (state_bits & (1 << 0)) != 0;
      exists = (state_bits & (1 << 1)) != 0;
    }
    Err(_) => panic!("Error in retrieving GetClockState"),
  }

  Ok(ClockInfo {
    id: clock_id,
    active,
    exists,
    rate_hz,
  })
}
