pub mod bcm2837_pl011;
#[cfg(feature = "host")]
pub mod mock;

use crate::common;
use crate::tty;
use arrayvec::ArrayVec;

// These will be set to 0 during bss zero-ing.
static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
static mut SET: bool = false;
// Only support single callback for now.
static mut RX_CALLBACK: core::mem::MaybeUninit<OnReceiveCallback> =
  core::mem::MaybeUninit::<OnReceiveCallback>::uninit();
static mut RX_CALLBACK_SET: bool = false;

struct Ops {
  getc: fn() -> u8,
  putc: fn(u8),
  interrupt_enable: core::option::Option<fn()>,
}

pub const PAYLOAD_SIZE: usize = 16;
pub type Payload = ArrayVec<u8, PAYLOAD_SIZE>;
pub type OnReceiveCallback = fn(Payload);

#[inline(always)]
pub fn getc() -> u8 {
  unsafe {
    assert!(SET, "UART not set");
    (OPS.assume_init_ref().getc)()
  }
}

#[inline(always)]
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

#[inline(always)]
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

fn on_receive(payload: Payload) {
  unsafe {
    if !RX_CALLBACK_SET {
      return;
    }
    RX_CALLBACK.assume_init_ref()(payload);
  }
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

pub fn interrupt_supported() -> bool {
  unsafe {
    assert!(SET, "UART not set");
    OPS.assume_init_ref().interrupt_enable.is_some()
  }
}

pub fn interrupt_enable() {
  unsafe {
    assert!(SET, "UART not set");
    OPS.assume_init_ref().interrupt_enable.unwrap()();
  }
}

pub fn set_receive_callback(cb: OnReceiveCallback) {
  unsafe {
    assert!(!RX_CALLBACK_SET, "RX Callback already set");
    RX_CALLBACK = core::mem::MaybeUninit::new(cb);
    RX_CALLBACK_SET = true;
  }
}
