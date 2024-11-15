use arrayvec::ArrayVec;
use core::mem::MaybeUninit;

pub mod bcm2837_interrupt;
mod macros;

// This is run in IRQ context. Don't do too much inside!
static mut HANDLERS: MaybeUninit<HandlerContainer> = MaybeUninit::uninit();
static mut OPS: core::mem::MaybeUninit<Ops> = core::mem::MaybeUninit::uninit();
static mut IMPL_SET: bool = false;

// An "IRQ domain" represents a group of IRQs. A controller can hold multiple
// IRQ domains, and a combination of (domain, number) constitutes a different
// IRQ. We call this combination as "IRQ channel".
// For example, BCM2837 controller controls 2 domain: Peripheral and ARM (base).
// IRQ number 1 on ARM domain points to ARM mailbox IRQ.
// IRQ number 1 on peripheral domain points to System timer 3.

struct IrqDomainBase(u8);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct IrqDomain(*const ());

unsafe impl core::marker::Sync for IrqDomain {}
impl IrqDomain {
  pub fn get(&self) -> *const () {
    self.0
  }
}

type HandlerContainer = ArrayVec<HandlerMeta, 128>;

struct HandlerMeta {
  channel: IrqChannel,
  handler: fn(),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct IrqChannel {
  pub domain: IrqDomain,
  pub number: u32,
}

struct Ops {
  mask_interrupt: fn(channel: IrqChannel),
  unmask_interrupt: fn(channel: IrqChannel),
  pub serve_interrupt: fn(handlers: &[HandlerMeta]),
}

pub fn set_handler(channel: IrqChannel, handler: fn()) {
  unsafe {
    assert!(IMPL_SET, "Impl not set");
    HANDLERS
      .assume_init_mut()
      .push(HandlerMeta { channel, handler });
  }
}

pub fn mask_interrupt(channel: IrqChannel) {
  unsafe {
    assert!(IMPL_SET, "Impl not set");
    (OPS.assume_init_ref().mask_interrupt)(channel);
  }
}

pub fn unmask_interrupt(channel: IrqChannel) {
  unsafe {
    assert!(IMPL_SET, "Impl not set");
    (OPS.assume_init_ref().unmask_interrupt)(channel);
  }
}

pub fn serve_interrupt() {
  unsafe {
    assert!(IMPL_SET, "Impl not set");
    (OPS.assume_init_ref().serve_interrupt)(
      HANDLERS.assume_init_ref().as_slice(),
    );
  }
}

fn register_device(ops: Ops) {
  unsafe {
    if !IMPL_SET {
      HANDLERS = MaybeUninit::<HandlerContainer>::new(HandlerContainer::new());
    }
    IMPL_SET = true;
    OPS = core::mem::MaybeUninit::new(ops);
  }
}
