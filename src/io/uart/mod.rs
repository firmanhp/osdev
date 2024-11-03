pub mod bcm2837_pl011;
#[cfg(feature = "host")]
pub mod mock;

use crate::common;
use crate::tty;

static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

struct Ops {
  getc: fn() -> u8,
  putc: fn(u8),
}

pub fn getc() -> u8 {
  unsafe {
    assert!(SET, "UART not set");
    (OPS.assume_init_ref().getc)()
  }
}

pub fn putc(ch: u8) {
  unsafe {
    assert!(SET, "UART not set");
    putc_unchecked(ch);
  }
}

#[inline]
fn putc_unchecked(ch: u8) {
  unsafe { (OPS.assume_init_ref().putc)(ch) }
}

pub fn puts(s: &str) {
  unsafe { assert!(SET, "UART not set") };
  for c in s.as_bytes() {
    putc_unchecked(*c);
  }
}

fn puts_ok(s: &str) -> Result<(), common::error::ErrorKind> {
  puts(s);
  Ok(())
}

fn register_device(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}

pub fn set_as_stream() {
  unsafe { assert!(SET, "UART not set") };
  common::stream::assign(common::stream::OutputOps { write: puts_ok })
}

pub fn as_tty_adapter() -> tty::TtyStreamAdapter {
  tty::TtyStreamAdapter {
    read_char: getc,
    write_char: putc,
  }
}

// Initialize device driver
#[macro_export]
macro_rules! init {
  ($device_name:ident) => {{
    use crate::io::uart::$device_name as device;
    device::device_init();
  }};
}

#[macro_export]
// Initialize device driver, and assign UART into stream subsystem.
macro_rules! init_with_stream {
  ($device_name:ident) => {{
    use crate::io::uart;
    use crate::io::uart::$device_name as device;
    device::device_init();
    uart::set_as_stream();
  }};
}

pub use init;
pub use init_with_stream;
