#![cfg_attr(feature = "device", no_std)]
#![cfg_attr(feature = "device", no_main)]

mod arch;
mod common;
mod container;
mod diagnostic;
mod io;
mod metadata;
mod panic;
mod syscall;
mod tty;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn kernel_main() -> ! {
  diagnostic::test_videocore_base_clock();
}
