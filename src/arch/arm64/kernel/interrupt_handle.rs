use crate::{common::stream, interrupt};

extern "C" {
  static _irq_vectors: [u8; 0];
}

#[no_mangle]
extern "C" fn on_irq() {
  stream::println!("Hey, an interrupt!");
  interrupt::serve_interrupt();
}

#[no_mangle]
extern "C" fn on_invalid_irq(irq_type: u64, esr_el1: u64, elr_el1: u64) -> ! {
  stream::println!(
    "Found invalid IRQ: type={}, esr_el1={:08X}, elr_el1={:08X}",
    irq_type,
    esr_el1,
    elr_el1
  );

  panic!("Invalid IRQ");
}

pub fn initialize() {
  unsafe {
    core::arch::asm!("msr vbar_el1, {0:x}", in(reg) _irq_vectors.as_ptr())
  };
  unsafe { core::arch::asm!("nop") };
}

pub fn enable_irq() {
  unsafe { core::arch::asm!("msr daifclr, #2") };
}

pub fn disable_irq() {
  unsafe { core::arch::asm!("msr daifset, #2") };
}
