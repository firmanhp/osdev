pub mod bcm2837_system_timer;

use crate::common::error;

// TODO: proper timer subsystem implementation.

// This is run in IRQ context. Don't do too much inside!
static mut IRQ_CALLBACK: core::mem::MaybeUninit<fn()> =
  core::mem::MaybeUninit::uninit();
static mut OPS: core::mem::MaybeUninit<Ops> = core::mem::MaybeUninit::uninit();

// These will be zeroed out.
static mut CALLBACK_SET: bool = false;
static mut IMPL_SET: bool = false;

struct Ops {
  set_timer: fn(jiffies: u32),
}

pub fn set_timer(jiffies: u32, callback: fn()) -> Result<(), error::ErrorKind> {
  unsafe {
    assert!(IMPL_SET, "Impl not set");
    if CALLBACK_SET {
      return Err(error::ErrorKind::ResourceBusy);
    }

    IRQ_CALLBACK = core::mem::MaybeUninit::new(callback);
    CALLBACK_SET = true;
    (OPS.assume_init_ref().set_timer)(jiffies);
  }
  Ok(())
}

fn do_callback() {
  unsafe {
    assert!(
      CALLBACK_SET,
      "do_callback was called on uninitialized callback"
    );
    // clear out first, then execute
    let callback = IRQ_CALLBACK.assume_init();
    IRQ_CALLBACK = core::mem::MaybeUninit::uninit();
    CALLBACK_SET = false;
    callback();
  }
}

fn register_device(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    IMPL_SET = true;
  };
}
