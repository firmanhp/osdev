use crate::common::bit;
use crate::common::error;
use crate::interrupt;
use crate::io::gpio;
use crate::io::mmio;
use crate::io::uart;

use super::interrupt_supported;

// BCM2837 implementation of UART0/PL011

pub struct InitParams {
  // Corresponding IRQ channel connected to this peripheral.
  pub irq_channel: interrupt::IrqChannel,
}

// Datasheet has typo on 'interrupt' (interupt).
struct Reg;
#[allow(dead_code)]
impl Reg {
  const BASE: u64 = 0x0020_10_00;
  const DR: u64 = Reg::BASE + 0x00; // Data Register
  const FR: u64 = Reg::BASE + 0x18; // Flag register
  const IBRD: u64 = Reg::BASE + 0x24; // Integer Baud rate divisor
  const FBRD: u64 = Reg::BASE + 0x28; // Fractional Baud rate divisor
  const LCRH: u64 = Reg::BASE + 0x2C; // Line Control register
  const CR: u64 = Reg::BASE + 0x30; // Control register
  const IFLS: u64 = Reg::BASE + 0x34; // Interupt FIFO Level Select Register
  const IMSC: u64 = Reg::BASE + 0x38; // Interupt Mask Set Clear Register
  const RIS: u64 = Reg::BASE + 0x3C; // Raw Interupt Status Register
  const MIS: u64 = Reg::BASE + 0x40; // Masked Interupt Status Register
  const ICR: u64 = Reg::BASE + 0x44; // Interupt Clear Register
  const DMACR: u64 = Reg::BASE + 0x48; // DMA Control Register
  const ITCR: u64 = Reg::BASE + 0x80; // Test Control register
  const ITIP: u64 = Reg::BASE + 0x84; // Integration test input reg
  const ITOP: u64 = Reg::BASE + 0x88; // Integration test output reg
  const TDR: u64 = Reg::BASE + 0x8C; // Test Data reg
}

struct Bit;
#[allow(dead_code)]
impl Bit {
  // CR control
  const CR_UARTEN: u32 = 1 << 0;
  const CR_LBE: u32 = 1 << 7;
  const CR_TXE: u32 = 1 << 8;
  const CR_RXE: u32 = 1 << 9;
  const CR_RTS: u32 = 1 << 11;
  const CR_RTSEN: u32 = 1 << 14;
  const CR_CTSEN: u32 = 1 << 15;

  // LCRH control
  const LCRH_BRK: u32 = 1 << 0;
  const LCRH_PEN: u32 = 1 << 1;
  const LCRH_EPS: u32 = 1 << 2;
  const LCRH_STP2: u32 = 1 << 3;
  const LCRH_FEN: u32 = 1 << 4;
  const LCRH_WLEN_5: u32 = 0b00 << 5;
  const LCRH_WLEN_6: u32 = 0b01 << 5;
  const LCRH_WLEN_7: u32 = 0b10 << 5;
  const LCRH_WLEN_8: u32 = 0b11 << 5;
  const LCRH_WLEN_SPS: u32 = 1 << 7;

  // ICR control
  const ICR_CTSMIC: u32 = 1 << 1;
  const ICR_RXIC: u32 = 1 << 4;
  const ICR_TXIC: u32 = 1 << 5;
  const ICR_RTIC: u32 = 1 << 6;
  const ICR_FEIC: u32 = 1 << 7;
  const ICR_PEIC: u32 = 1 << 8;
  const ICR_BEIC: u32 = 1 << 9;
  const ICR_OEIC: u32 = 1 << 10;
  // Clear all
  const ICR_ALL: u32 = Bit::ICR_CTSMIC
    | Bit::ICR_RXIC
    | Bit::ICR_TXIC
    | Bit::ICR_RTIC
    | Bit::ICR_FEIC
    | Bit::ICR_PEIC
    | Bit::ICR_BEIC
    | Bit::ICR_OEIC;

  // IMSC control
  const IMSC_CTSMIM: u32 = 1 << 1;
  const IMSC_RXIM: u32 = 1 << 4;
  const IMSC_TXIM: u32 = 1 << 5;
  const IMSC_RTIM: u32 = 1 << 6;
  const IMSC_FEIM: u32 = 1 << 7;
  const IMSC_PEIM: u32 = 1 << 8;
  const IMSC_BEIM: u32 = 1 << 9;
  const IMSC_OEIM: u32 = 1 << 10;
  const IMSC_RESERVED: u32 =
    bit::bit_range::<31, 11>() | (1 << 3) | (1 << 2) | (1 << 0);
  // Mask all
  const IMSC_ALL: u32 = !Bit::IMSC_RESERVED
    & (Bit::IMSC_CTSMIM
      | Bit::IMSC_RXIM
      | Bit::IMSC_TXIM
      | Bit::IMSC_RTIM
      | Bit::IMSC_FEIM
      | Bit::IMSC_PEIM
      | Bit::IMSC_BEIM
      | Bit::IMSC_OEIM);

  // FR control
  const FR_CTS: u32 = 1 << 0;
  const FR_BUSY: u32 = 1 << 3;
  const FR_RXFE: u32 = 1 << 4;
  const FR_TXFF: u32 = 1 << 5;
  const FR_RXFF: u32 = 1 << 6;
  const FR_TXFE: u32 = 1 << 7;

  // RIS (raw interrupt)
  const RIS_OERIS: u32 = 1 << 10;
  const RIS_BERIS: u32 = 1 << 9;
  const RIS_PERIS: u32 = 1 << 8;
  const RIS_FERIS: u32 = 1 << 7;
  const RIS_RTRIS: u32 = 1 << 6;
  const RIS_TXRIS: u32 = 1 << 5;
  const RIS_RXRIS: u32 = 1 << 4;
  const RIS_CTSRMIS: u32 = 1 << 1;
  const RIS_RESERVED: u32 =
    bit::bit_range::<31, 11>() | (1 << 3) | (1 << 2) | (1 << 0);

  // Masked interrupt status
  const MIS_OEMIS: u32 = 1 << 10;
  const MIS_BEMIS: u32 = 1 << 9;
  const MIS_PEMIS: u32 = 1 << 8;
  const MIS_FEMIS: u32 = 1 << 7;
  const MIS_RTMIS: u32 = 1 << 6;
  const MIS_TXMIS: u32 = 1 << 5;
  const MIS_RXMIS: u32 = 1 << 4;
  const MIS_CTSMMIS: u32 = 1 << 1;
  const MIS_RESERVED: u32 =
    bit::bit_range::<31, 11>() | (1 << 3) | (1 << 2) | (1 << 0);
}

// Initialized during initialize()
static mut IRQ_CHANNEL: core::mem::MaybeUninit<interrupt::IrqChannel> =
  core::mem::MaybeUninit::uninit();

// Only handle receivefor now
static HANDLE_INTERRUPTS: u32 = Bit::IMSC_RXIM /*| Bit::IMSC_TXIM*/;

// Initialize device driver.
pub fn initialize(params: InitParams) {
  controller_setup();
  interrupt_setup(params.irq_channel);
  // Register the device to UART subsystem
  uart::register_device(uart::Ops {
    getc,
    putc,
    interrupt_enable: Some(interrupt_enable),
  });
}

fn controller_setup() {
  // https://elinux.org/RPi_BCM2835_GPIOs
  // Func0 is TXD0/RXD0
  gpio::set_function((1 << 14) | (1 << 15), gpio::Function::Func0);
  // Disable pull up/down for GPIO pin 14, 15.
  gpio::set_pull_mode((1 << 14) | (1 << 15), gpio::PullMode::Disabled);
  // Disable everything first
  mmio::write(Reg::CR, 0x00);
  // Clear pending interrupts
  mmio::write(Reg::ICR, Bit::ICR_ALL);

  // The baud rate divisor is calculated as follows:
  // Baud rate divisor BAUDDIV = (FUARTCLK/(16 Baud rate))
  // where FUARTCLK is the UART reference clock frequency.
  // The BAUDDIV is comprised of the integer value IBRD and the fractional value FBRD.
  // let base_clk_rate_hz = clock::get_clock_info(clock::ClockId::UART)
  //   .and_then(|c| Ok(c.rate_hz))
  //   .unwrap_or(3000000);

  // Set baud rate to 115200bps
  // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
  // only first 16 bits
  // let divider_integer: u32 = base_clk_rate_hz / (16 * 115200);
  // mmio::write(Reg::IBRD, divider_integer & 0x0000_FFFF);
  // Get the fractional part without floating point manipulation
  // let divider_fractional = {
  //   let remainder = base_clk_rate_hz % (16 * 115200);
  //   // Bring the first 6 bits to integer part
  //   (remainder * (1 << 6)) / (16 * 115200)
  // };
  // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
  // mmio::write(Reg::FBRD, divider_fractional & 0x00000_003F);

  // Enable FIFO
  // 8 bit data transmission (1 stop bit, no parity).
  mmio::write(Reg::LCRH, Bit::LCRH_FEN | Bit::LCRH_WLEN_8);
  // Mask all interrupts.
  // For whatever reason the "Mask" here enables the interrupt, so we should
  // write 0.
  // mmio::write(Reg::IMSC, Bit::IMSC_ALL);
  mmio::write(Reg::IMSC, 0);
  // Enable UART, receive and transfer.
  mmio::write(Reg::CR, Bit::CR_UARTEN | Bit::CR_RXE | Bit::CR_TXE);
}

fn getc() -> u8 {
  // Wait for any inputs
  while mmio::read(Reg::FR) & Bit::FR_RXFE != 0 {}
  return (mmio::read(Reg::DR) & 0xFF) as u8;
}

fn putc(c: u8) {
  // Wait for TX FIFO not empty
  while mmio::read(Reg::FR) & Bit::FR_TXFF != 0 {}
  mmio::write(Reg::DR, c as u32);
}

fn interrupt_enable() {
  unsafe { interrupt::unmask_interrupt(IRQ_CHANNEL.assume_init()) };
}

fn interrupt_setup(irq_channel: interrupt::IrqChannel) {
  // Enable relevant interrupts
  // only Receive
  // Refer to controller_setup part, this enables only receive
  mmio::write(Reg::IMSC, HANDLE_INTERRUPTS);
  unsafe { IRQ_CHANNEL = core::mem::MaybeUninit::new(irq_channel) };
  interrupt::set_handler(irq_channel, handle_irq);
}

fn handle_irq() {
  // Only handle unmasked interrupt
  let masked_interrupts = mmio::read(Reg::MIS);
  let raw_interrupts = mmio::read(Reg::RIS);
  let interrupts_unmasked =
    !Bit::RIS_RESERVED & masked_interrupts & raw_interrupts;
  // crate::common::stream::println!("Masked interrupt: {:b}", masked_interrupts);
  // crate::common::stream::println!("Raw interrupt: {:b}", raw_interrupts);
  if interrupts_unmasked == 0 {
    return;
  }

  if interrupts_unmasked & Bit::RIS_RXRIS > 0 {
    // Clear first then pull whatever we have
    mmio::write(Reg::ICR, Bit::ICR_RXIC);
    on_rx_fifo();
  }

  if interrupts_unmasked != Bit::RIS_RXRIS {
    crate::common::stream::println!(
      "UNHANDLED INTERRUPT: {:b}",
      interrupts_unmasked
    );
    panic!("Unhandled interrupt {}", interrupts_unmasked);
  }
}

// Given a "big enough" payload size, we should not miss any data in the
// fifo. If a new data comes after we clear the interrupt, a new interrupt
// will be fired to serve that data. If that new data was also picked up by
// the same interrupt, then the next interrupt will be no-op.
fn on_rx_fifo() {
  let mut payload: uart::Payload = uart::Payload::new();
  // while not full and not empty
  while !payload.is_full() && (mmio::read(Reg::FR) & Bit::FR_RXFE == 0) {
    payload.push((mmio::read(Reg::DR) & 0xFF) as u8);
  }

  if !payload.is_empty() {
    uart::on_receive(payload);
  }
}
