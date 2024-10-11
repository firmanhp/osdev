use crate::io::mailbox;

// members are actually read in unsafe way
#[allow(dead_code)]
#[repr(packed)]
pub struct GetClockState {
  pub clock_id: u32,
}

impl mailbox::MessageTag for GetClockState {
  fn id(&self) -> u32 {
    0x00030001
  }
}

// members are actually read in unsafe way
#[allow(dead_code)]
#[repr(packed)]
pub struct GetClockRate {
  pub clock_id: u32,
}

impl mailbox::MessageTag for GetClockRate {
  fn id(&self) -> u32 {
    0x00030002
  }
}
