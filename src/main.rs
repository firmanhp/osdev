#![cfg_attr(feature = "device", no_std)]
#![cfg_attr(feature = "device", no_main)]

mod board_info;
mod boot;
mod common;
mod container;
mod diagnostic;
mod io;
mod synchronization;

use io::gpio;
use io::mmio;
use io::uart;

#[cfg(feature = "device")]
#[no_mangle]
#[allow(unused_variables)]
extern "C" fn kernel_main() {
  mmio::init();
  uart::pl011_init();

  diagnostic::test_uart();
}

#[cfg(not(feature = "device"))]
fn main() {}

#[cfg(feature = "device")]
fn on_panic_impl(info: &core::panic::PanicInfo) -> ! {
  use common::stream;
  const GPIO: u64 = 1 << 27;

  uart::pl011_init();

  stream::println!("PANIC: {}", info.message().as_str().unwrap_or("No message"));

  if let Some(location) = info.location() {
    stream::println!("{}:{}", location.file(), location.line());
  }

  gpio::set_function(GPIO, gpio::Function::Output);
  gpio::set_pull_mode(GPIO, gpio::PullMode::Disabled);

  loop {
    gpio::output_set(GPIO);
    synchronization::sleep(250_000);
    gpio::output_clear(GPIO);
    synchronization::sleep(250_000);
  }
}

#[panic_handler]
#[cfg(feature = "device")]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
  on_panic_impl(info)
}
