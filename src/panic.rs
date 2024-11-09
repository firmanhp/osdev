use crate::common::stream;
use crate::common::synchronization;
use crate::io::gpio;

static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

pub struct Ops {
  pub pre_handler: fn(),
  pub post_handler: fn() -> !,
}

#[panic_handler]
#[cfg(feature = "device")]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
  unsafe {
    if !SET {
      // Can't do anything here
      loop {}
    }
    (OPS.assume_init_ref().pre_handler)();
  }

  stream::println!(
    "PANIC: {}",
    info.message().as_str().unwrap_or("No message")
  );

  if let Some(location) = info.location() {
    stream::println!("{}:{}", location.file(), location.line());
  }

  unsafe { (OPS.assume_init_ref().post_handler)() };
}

pub fn set_handler(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}
