pub mod arm64_generic_mmio;

pub struct Ops {
  write: fn(address: u64, data: u32),
  read: fn(address: u64) -> u32,
}

static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

#[inline(always)]
pub fn write(addr: u64, data: u32) {
  unsafe {
    assert!(SET, "MMIO not set");
    (OPS.assume_init_ref().write)(addr, data);
  }
}

#[inline(always)]
pub fn read(addr: u64) -> u32 {
  unsafe {
    assert!(SET, "UART not set");
    (OPS.assume_init_ref().read)(addr)
  }
}

fn register_device(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}

// Initialize device driver
#[macro_export]
macro_rules! mmio_init {
  ($device_name:ident, $($args:expr),*) => {{
    use crate::io::mmio::$device_name as device;
    device::device_init($($args,)*);
  }};
}

pub use mmio_init;
