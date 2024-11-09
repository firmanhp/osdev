use crate::arch::arm64::asm;
use crate::io::mmio;

static mut BASE_OFFSET: u64 = 0x0;

pub fn device_init(base_address: u64) {
  unsafe { BASE_OFFSET = base_address };
  mmio::register_device(mmio::Ops { write, read });
}

fn write(addr: u64, data: u32) {
  // let's revisit later on the barrier...
  asm::barrier::data_memory!("sy");
  unsafe { core::ptr::write_volatile((BASE_OFFSET + addr) as *mut u32, data) }
  asm::barrier::data_memory!("sy");
}

fn read(addr: u64) -> u32 {
  // let's revisit later on the barrier...
  asm::barrier::data_memory!("sy");
  let ret =
    unsafe { core::ptr::read_volatile((BASE_OFFSET + addr) as *mut u32) };
  asm::barrier::data_memory!("sy");
  ret
}
