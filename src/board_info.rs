use core::arch::asm;

#[allow(non_camel_case_types)]
pub enum BoardType {
  UNKNOWN,
  PI_1,
  PI_2,
  PI_3,
  PI_4,
}

// https://wiki.osdev.org/Detecting_Raspberry_Pi_Board
#[cfg(target_arch = "aarch64")]
pub fn get_board_info() -> BoardType {
  let mut id: u32;
  // Move System Register
  // https://developer.arm.com/documentation/ddi0602/2024-06/Base-Instructions/MRS--Move-System-register-to-general-purpose-register-?lang=en
  // MIDR_EL1
  // https://developer.arm.com/documentation/ddi0601/2024-06/External-Registers/MIDR-EL1--Main-ID-Register?lang=en
  unsafe {
    asm!("mrs {0:x}, midr_el1", out(reg) id);
  }
  let part_num: u32 = (id >> 4) & 0xFFF;
  match part_num {
    // PI_1 and PI_2 
    0xB76 => BoardType::PI_1,
    0xC07 => BoardType::PI_2,
    0xD03 => BoardType::PI_3,
    0xD08 => BoardType::PI_4,
    _ => BoardType::UNKNOWN,
  }
}
