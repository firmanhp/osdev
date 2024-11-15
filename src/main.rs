#![cfg_attr(feature = "device", no_std)]
#![cfg_attr(feature = "device", no_main)]

mod arch;
mod common;
mod container;
mod diagnostic;
mod interrupt;
mod io;
mod metadata;
mod panic;
mod syscall;
mod timer;
mod tty;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn kernel_main() -> ! {
  diagnostic::test_interrupt();
}
