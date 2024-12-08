use crate::interrupt;
use crate::interrupt::bcm2837_interrupt;
use crate::io::gpio;
use crate::io::mmio;
use crate::io::uart;
use crate::timer;

use crate::arch::arm64::kernel::interrupt_handle;
use crate::arch::arm64::vendor::broadcom::bcm2837_raspberrypi_3b::panic;
use crate::arch::arm64::vendor::broadcom::bcm_raspberrypi_common;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn board_setup() {
  panic::initialize();
  // Dependency: MMIO -> GPIO -> UART
  mmio::arm64_generic_mmio::initialize(
    bcm_raspberrypi_common::mmio::base_address(),
  );
  // interrupt requires MMIO
  bcm2837_interrupt::initialize();
  gpio::bcm2837_gpio::initialize();
  // UART requires GPIO
  uart::bcm2837_pl011::initialize(uart::bcm2837_pl011::InitParams {
    irq_channel: interrupt::IrqChannel {
      domain: bcm2837_interrupt::domains::PERIPHERAL,
      number: 57,
    },
  });
  uart::set_as_stream();
  // network requires MMIO, mailbox
  bcm_raspberrypi_common::network::initialize();
  // board_Info requires MMIO, mailbox
  bcm_raspberrypi_common::board_info::initialize();
  timer::bcm2837_system_timer::initialize(
    timer::bcm2837_system_timer::InitParams {
      irq_channel: interrupt::IrqChannel {
        domain: bcm2837_interrupt::domains::PERIPHERAL,
        number: 1,
      },
    },
  );
}
