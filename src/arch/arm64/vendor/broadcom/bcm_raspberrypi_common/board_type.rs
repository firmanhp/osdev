#[repr(u32)]
pub enum RaspiBoardType {
  Unknown = 0,
  Pi1 = 0x0B76,
  Pi2 = 0x0C07,
  Pi3 = 0x0D03,
  Pi4 = 0x0D08,
}

impl From<u32> for RaspiBoardType {
  fn from(val: u32) -> RaspiBoardType {
    match val {
      x if x == RaspiBoardType::Pi1 as u32 => RaspiBoardType::Pi1,
      x if x == RaspiBoardType::Pi2 as u32 => RaspiBoardType::Pi2,
      x if x == RaspiBoardType::Pi3 as u32 => RaspiBoardType::Pi3,
      x if x == RaspiBoardType::Pi4 as u32 => RaspiBoardType::Pi4,
      _ => RaspiBoardType::Unknown,
    }
  }
}

impl core::fmt::Display for RaspiBoardType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      RaspiBoardType::Unknown => write!(f, "UNKNOWN"),
      RaspiBoardType::Pi1 => write!(f, "Pi1"),
      RaspiBoardType::Pi2 => write!(f, "Pi2"),
      RaspiBoardType::Pi3 => write!(f, "Pi3"),
      RaspiBoardType::Pi4 => write!(f, "Pi4"),
    }
  }
}

// https://wiki.osdev.org/Detecting_Raspberry_Pi_Board
pub fn raspi_board_type() -> RaspiBoardType {
  let mut id: u32;
  // Move System Register
  // https://developer.arm.com/documentation/ddi0602/2024-06/Base-Instructions/MRS--Move-System-register-to-general-purpose-register-?lang=en
  // MIDR_EL1
  // https://developer.arm.com/documentation/ddi0601/2024-06/External-Registers/MIDR-EL1--Main-ID-Register?lang=en
  unsafe { core::arch::asm!("mrs {0:x}, midr_el1", out(reg) id) };
  let part_num: u32 = (id >> 4) & 0xFFF;
  return RaspiBoardType::from(part_num);
}
