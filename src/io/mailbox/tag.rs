use crate::io::mailbox;
use mailbox::macros;

#[repr(u32)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum TagId {
  // VideoCore
  VideocoreGetFirmwareRevision = 0x00000001,
  // Hardware
  HwGetBoardModel = 0x00010001,
  HwGetBoardRevision = 0x00010002,
  HwGetBoardMacAddress = 0x00010003,
  HwGetBoardSerial = 0x00010004,
  HwGetArmMemory = 0x00010005,
  HwGetVideocoreMemory = 0x00010006,
  HwGetClocks = 0x00010007,
  GetClockState = 0x00030001,
  SetClockState = 0x00038001,
  GetClockRate = 0x00030002,
}

// Message tag buffer must be 4 byte aligned.
// FIXME: could have better design
pub trait MessageTag {
  // Returns the size of the tag struct in bytes.
  fn size_bytes(&self) -> usize;
  // Returns the size of the payload in bytes.
  fn payload_size_bytes(&self) -> usize;
  // Value buffer of the tag.
  fn buf(&self) -> &[u8] {
    unsafe {
      ::core::slice::from_raw_parts(
        (self as *const Self) as *const u8,
        self.size_bytes(),
      )
    }
  }
}

macros::make_tag!(
  GetClockState,
  TagId::GetClockState,
  request { clock_id: u32 },
  response {
    clock_id: u32,
    state_bits: u32
  }
);

macros::make_tag!(
  GetClockRate,
  TagId::GetClockRate,
  request { clock_id: u32 },
  response {
    clock_id: u32,
    rate_hz: u32
  }
);

// VideoCore
macros::make_tag!(
  VideocoreGetFirmwareRevision,
  TagId::VideocoreGetFirmwareRevision,
  request {},
  response { firmware_rev: u32 }
);

// Hardware
macros::make_tag!(
  HwGetBoardModel,
  TagId::HwGetBoardModel,
  request {},
  response { board_model: u32 }
);

macros::make_tag!(
  HwGetBoardRevision,
  TagId::HwGetBoardRevision,
  request {},
  response { board_rev: u32 }
);

macros::make_tag!(
  HwGetBoardMacAddress,
  TagId::HwGetBoardMacAddress,
  request {},
  response {
    // In network byte order
    mac_address: [u8; 6]
  }
);

macros::make_tag!(
  HwGetBoardSerial,
  TagId::HwGetBoardSerial,
  request {},
  response { serial: u64 }
);

macros::make_tag!(
  HwGetArmMemory,
  TagId::HwGetArmMemory,
  request {},
  response {
    base_address: u32,
    size_bytes: u32
  }
);

macros::make_tag!(
  HwGetVideocoreMemory,
  TagId::HwGetVideocoreMemory,
  request {},
  response {
    base_address: u32,
    size_bytes: u32
  }
);

macros::make_tag!(
  HwGetClocks,
  TagId::HwGetClocks,
  request {},
  response {
    // Allocate 64 entries. 32 clocks.
    // even index is parent clock id, odd is clock id.
    parent_clock_pair: [u32; 64]
  }
);
