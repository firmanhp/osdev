use crate::arch::arm64::vendor::broadcom::bcm_raspberrypi_common::board_type;

pub fn base_address() -> u64 {
  use board_type::RaspiBoardType;
  match board_type::raspi_board_type() {
    RaspiBoardType::Pi2 | RaspiBoardType::Pi3 => 0x3F000000,
    RaspiBoardType::Pi4 => 0xFE000000,
    // (should be unreachable)
    // for raspi1, raspi zero etc.
    _ => 0x20000000,
  }
}
