use crate::asm::barrier;
use crate::meta::board_info;

static mut BASE_ADDR: u64 = 0;

pub fn init() {
  let board_type = board_info::raspi_board_type();
  unsafe {
    use board_info::RaspiBoardType;
    match board_type {
      RaspiBoardType::Pi2 | RaspiBoardType::Pi3 => BASE_ADDR = 0x3F000000,
      RaspiBoardType::Pi4 => BASE_ADDR = 0xFE000000,
      // (should be unreachable)
      // for raspi1, raspi zero etc.
      _ => BASE_ADDR = 0x20000000,
    }
  }
}

#[inline(always)]
pub fn write(addr: u64, data: u32) {
  // let's revisit later on the barrier...
  barrier::aarch64::data_memory!("sy");
  unsafe { core::ptr::write_volatile((BASE_ADDR + addr) as *mut u32, data) }
  barrier::aarch64::data_memory!("sy");
}

#[inline(always)]
pub fn read(addr: u64) -> u32 {
  // let's revisit later on the barrier...
  barrier::aarch64::data_memory!("sy");
  let ret = unsafe { core::ptr::read_volatile((BASE_ADDR + addr) as *mut u32) };
  barrier::aarch64::data_memory!("sy");
  ret
}
