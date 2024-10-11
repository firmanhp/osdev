use crate::board_info;

static mut BASE_ADDR: u64 = 0;

pub fn init() {
  let board_type = board_info::get_board_info();
  unsafe {
    use board_info::BoardType;
    match board_type {
      BoardType::PI_2 | BoardType::PI_3 => BASE_ADDR = 0x3F000000,
      BoardType::PI_4 => BASE_ADDR = 0xFE000000,
      // (should be unreachable)
      // for raspi1, raspi zero etc.
      _ => BASE_ADDR = 0x20000000,
    }
  }
}

#[inline(always)]
pub fn write(addr: u64, data: u32) {
  unsafe { core::ptr::write_volatile((BASE_ADDR + addr) as *mut u32, data) }
}

#[inline(always)]
pub fn read(addr: u64) -> u32 {
  unsafe { core::ptr::read_volatile((BASE_ADDR + addr) as *mut u32) }
}
