// BCM2837 System timer implementation
// This timer serves 4 channel i.e. we can program multiple timers with
// different handlers From the datasheet we can only use channel 1 and 3. It is
// programmed through Compare N registers.

// Currently only operates on channel 1.

use crate::io::mmio;
use crate::{interrupt, timer};

pub struct InitParams {
  // Corresponding IRQ channel connected to this peripheral.
  pub irq_channel: interrupt::IrqChannel,
}

static mut IRQ_CHANNEL: core::mem::MaybeUninit<interrupt::IrqChannel> =
  core::mem::MaybeUninit::uninit();

struct Reg;
#[allow(dead_code)]
impl Reg {
  const ST_BASE: u64 = 0x0000_3000;
  const ST_CS: u64 = Reg::ST_BASE + 0x00; // Control/Status
  const ST_CLO: u64 = Reg::ST_BASE + 0x4; // Counter Lower 32 bits
  const ST_CHI: u64 = Reg::ST_BASE + 0x8; // Counter Higher 32 bits
  const ST_C0: u64 = Reg::ST_BASE + 0xC; // Compare 0 (unused)
  const ST_C1: u64 = Reg::ST_BASE + 0x10; // Compare 1
  const ST_C2: u64 = Reg::ST_BASE + 0x14; // Compare 2 (unused)
  const ST_C3: u64 = Reg::ST_BASE + 0x18; // Compare 3
}

struct Bit;
#[allow(dead_code)]
impl Bit {
  // ST_CS control
  // The M0-3 fields contain the free-running counter match status. Write a one
  // to the relevant bit to clear the match detect status bit and the
  // corresponding interrupt request line.
  const ST_CS_M0: u32 = 1 << 0;
  const ST_CS_M1: u32 = 1 << 1;
  const ST_CS_M2: u32 = 1 << 2;
  const ST_CS_M3: u32 = 1 << 3;
}

fn set_timer(jiffies: u32) {
  unsafe { interrupt::mask_interrupt(IRQ_CHANNEL.assume_init()) };
  let current_counter = mmio::read(Reg::ST_CLO);
  // crate::common::stream::println!("Counter: {}, {} ", current_counter, jiffies);
  mmio::write(Reg::ST_C1, current_counter + jiffies);
  unsafe { interrupt::unmask_interrupt(IRQ_CHANNEL.assume_init()) };
}

fn handle_irq() {
  unsafe { interrupt::mask_interrupt(IRQ_CHANNEL.assume_init()) };
  mmio::write(Reg::ST_CS, Bit::ST_CS_M1);
  timer::do_callback();
}

pub fn initialize(params: InitParams) {
  unsafe { IRQ_CHANNEL = core::mem::MaybeUninit::new(params.irq_channel) };
  interrupt::set_handler(params.irq_channel, handle_irq);
  timer::register_device(timer::Ops { set_timer });
}
