use crate::io::gpio;
use crate::io::mmio;
use crate::io::uart;

use crate::arch::arm64::vendor::broadcom::bcm_raspberrypi_common;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn board_setup() {
  // Dependency: MMIO -> GPIO -> UART
  mmio::mmio_init!(
    arm64_generic_mmio,
    bcm_raspberrypi_common::mmio::base_address()
  );
  gpio::gpio_init!(bcm2837_gpio);
  uart::uart_init_with_stream!(bcm2837_pl011);
  // network requires MMIO, mailbox
  bcm_raspberrypi_common::network::initialize();
  // board_Info requires MMIO, mailbox
  bcm_raspberrypi_common::board_info::initialize();
}
