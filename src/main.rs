#![cfg_attr(feature = "device", no_std)]
#![cfg_attr(feature = "device", no_main)]

mod asm;
mod boot;
mod common;
mod container;
mod diagnostic;
mod io;
mod meta;
mod synchronization;
mod syscall;
mod tty;

use io::gpio;
use io::mmio;
use io::uart;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn kernel_main() -> ! {
  // Dependency: MMIO -> GPIO -> UART
  mmio::init();
  gpio::gpio_init!(bcm2837_gpio);
  uart::uart_init!(bcm2837_pl011);
  diagnostic::test_board_info();
}

#[panic_handler]
#[cfg(feature = "device")]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
  use common::stream;
  const GPIO: u64 = 1 << 27;

  uart::uart_init!(bcm2837_pl011);

  stream::println!(
    "PANIC: {}",
    info.message().as_str().unwrap_or("No message")
  );

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
