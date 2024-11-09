use crate::arch::arm64::vendor::broadcom::bcm_raspberrypi_common;
use crate::common::synchronization;
use crate::io::gpio;
use crate::io::mmio;
use crate::io::uart;
use crate::panic;

fn pre_handler() {
  // pray that these never panic
  // set up required stuffs to be able to print
  mmio::mmio_init!(
    arm64_generic_mmio,
    bcm_raspberrypi_common::mmio::base_address()
  );
  gpio::gpio_init!(bcm2837_gpio);
  uart::uart_init_with_stream!(bcm2837_pl011);
}

fn post_handler() -> ! {
  const GPIO: u64 = 1 << 27;
  gpio::set_function(GPIO, gpio::Function::Output);
  gpio::set_pull_mode(GPIO, gpio::PullMode::Disabled);

  loop {
    gpio::output_set(GPIO);
    synchronization::sleep(250_000);
    gpio::output_clear(GPIO);
    synchronization::sleep(250_000);
  }
}

pub fn initialize() {
  panic::set_handler(panic::Ops {
    pre_handler,
    post_handler,
  });
}
