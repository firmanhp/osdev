use crate::common::bit::bit;
use crate::common::bit::bit_of;
use crate::interrupt;
use crate::interrupt_declare_domains;
use crate::io::mmio;

// IRQ domains
interrupt_declare_domains!(ARM, PERIPHERAL);

struct Reg;
#[allow(dead_code)]
impl Reg {
  const IRQ_BASE: u64 = 0x0000_B000;
  const IRQ_BASIC_PENDING: u64 = Reg::IRQ_BASE + 0x200; // IRQ basic pending
  const IRQ_PENDING_1: u64 = Reg::IRQ_BASE + 0x204; // IRQ pending 1
  const IRQ_PENDING_2: u64 = Reg::IRQ_BASE + 0x208; // IRQ pending 2
  const IRQ_FIQ_CTRL: u64 = Reg::IRQ_BASE + 0x20C; // FIQ control
  const IRQ_EN_1: u64 = Reg::IRQ_BASE + 0x210; // Enable IRQs 1
  const IRQ_EN_2: u64 = Reg::IRQ_BASE + 0x214; // Enable IRQs 2
  const IRQ_EN_BASIC: u64 = Reg::IRQ_BASE + 0x218; // Enable Basic IRQs
  const IRQ_DI_1: u64 = Reg::IRQ_BASE + 0x21C; // Disable IRQs 1
  const IRQ_DI_2: u64 = Reg::IRQ_BASE + 0x220; // Disable IRQs 2
  const IRQ_DI_BASIC: u64 = Reg::IRQ_BASE + 0x224; // Disable Basic IRQs
}

struct Bit;
#[allow(dead_code)]
impl Bit {
  // ARM peripherals interrupts table.
  // Spread within IRQ 1, 2
  // The table has many empty entries. These should not be enabled as they will
  // interfere with the GPU operaiton.
  const IRQ_1_SYSTEM_TIMER_1: u32 = bit::<1>();
  const IRQ_1_SYSTEM_TIMER_3: u32 = bit::<3>();
  const IRQ_1_USB: u32 = bit::<9>();
  const IRQ_1_AUX: u32 = bit::<29>();
  const IRQ_2_I2C_SPI_SLAVE: u32 = bit::<11>();
  const IRQ_2_PWA_0: u32 = bit::<13>();
  const IRQ_2_PWA_1: u32 = bit::<14>();
  const IRQ_2_SMI: u32 = bit::<16>();
  const IRQ_2_GPIO_0: u32 = bit::<17>();
  const IRQ_2_GPIO_1: u32 = bit::<18>();
  const IRQ_2_GPIO_2: u32 = bit::<19>();
  const IRQ_2_GPIO_3: u32 = bit::<20>();
  const IRQ_2_I2C: u32 = bit::<21>();
  const IRQ_2_SPI: u32 = bit::<22>();
  const IRQ_2_PCM: u32 = bit::<23>();
  const IRQ_2_UART: u32 = bit::<25>();

  const IRQ_BASIC_ARM_TIMER: u32 = bit::<0>();
  const IRQ_BASIC_ARM_MBOX: u32 = bit::<1>();
  const IRQ_BASIC_ARM_DOORBELL_0: u32 = bit::<2>();
  const IRQ_BASIC_ARM_DOORBELL_1: u32 = bit::<3>();
  // Or GPU1 halted if bit 10 of control register 1 is set
  const IRQ_BASIC_GPU_HALTED_0: u32 = bit::<4>();
  const IRQ_BASIC_GPU_HALTED_1: u32 = bit::<5>();
  const IRQ_BASIC_ILLEGAL_ACC_0: u32 = bit::<6>();
  const IRQ_BASIC_ILLEGAL_ACC_1: u32 = bit::<7>();

  // One or more bits set in pending register 1/2
  const IRQ_PERIP_REG_1: u32 = bit::<8>();
  const IRQ_PERIP_REG_2: u32 = bit::<9>();

  // These GPUs are actually from the IRQ table i.e. IRQ_PENDING_1,
  // IRQ_PENDING_2.
  // IRQ_BASIC_PEND_REG_1/2 will be set if one of the IRQs inside
  // IRQ_PENDING_1/2 is pending, EXCEPT for these below.
  const IRQ_BASIC_PEND_GPU_7: u32 = bit::<10>();
  const IRQ_BASIC_PEND_GPU_9: u32 = bit::<11>(); // IRQ_1_USB
  const IRQ_BASIC_PEND_GPU_10: u32 = bit::<12>();
  const IRQ_BASIC_PEND_GPU_18: u32 = bit::<13>();
  const IRQ_BASIC_PEND_GPU_19: u32 = bit::<14>();
  const IRQ_BASIC_PEND_GPU_53: u32 = bit::<15>(); // IRQ_2_I2C
  const IRQ_BASIC_PEND_GPU_54: u32 = bit::<16>(); // IRQ_2_SPI
  const IRQ_BASIC_PEND_GPU_55: u32 = bit::<17>(); // IRQ_2_PCM
  const IRQ_BASIC_PEND_GPU_56: u32 = bit::<18>();
  const IRQ_BASIC_PEND_GPU_57: u32 = bit::<19>(); // IRQ_2_UART
  const IRQ_BASIC_PEND_GPU_62: u32 = bit::<20>();

  const FIQ_EN: u32 = bit::<7>();
  const FIQ_SOURCE_LSB: u8 = 0;
  const FIQ_SOURCE_MSB: u8 = 6;
}

pub fn initialize() {
  interrupt::register_device(interrupt::Ops {
    mask_interrupt,
    unmask_interrupt,
    serve_interrupt,
  });
}

fn mask_interrupt(channel: interrupt::IrqChannel) {
  if channel.domain == domains::ARM {
    assert!(
      channel.number < 8,
      "Invalid ARM IRQ number {}",
      channel.number
    );
    mmio::write(Reg::IRQ_DI_BASIC, 1 << channel.number);
    return;
  }
  if channel.domain == domains::PERIPHERAL {
    assert!(
      valid_irq_peripheral_number(channel.number),
      "Invalid peripheral IRQ number {}",
      channel.number
    );
    match channel.number {
      0..32 => mmio::write(Reg::IRQ_DI_1, 1 << channel.number),
      32..64 => mmio::write(Reg::IRQ_DI_2, 1 << (channel.number - 32)),
      _ => panic!("Unknown number {}", channel.number),
    }
    return;
  }
  panic!("Unknown domain");
}

fn unmask_interrupt(channel: interrupt::IrqChannel) {
  if channel.domain == domains::ARM {
    assert!(
      channel.number < 8,
      "Invalid ARM IRQ number {}",
      channel.number
    );
    mmio::write(Reg::IRQ_EN_BASIC, 1 << channel.number);
    return;
  }

  if channel.domain == domains::PERIPHERAL {
    assert!(
      valid_irq_peripheral_number(channel.number),
      "Invalid peripheral IRQ number {}",
      channel.number
    );
    match channel.number {
      0..32 => mmio::write(Reg::IRQ_EN_1, 1 << channel.number),
      32..64 => mmio::write(Reg::IRQ_EN_2, 1 << (channel.number - 32)),
      _ => panic!("Unknown number {}", channel.number),
    }
    return;
  }
  panic!("Unknown domain");
}

fn serve_interrupt(handlers: &[interrupt::HandlerMeta]) {
  let b = mmio::read(Reg::IRQ_BASIC_PENDING);
  let p1 = mmio::read(Reg::IRQ_PENDING_1);
  let p2 = mmio::read(Reg::IRQ_PENDING_2);
  // let handle_if = |bit_idx: u8, domain: interrupt::IrqDomain, number: u32| {};

  let h = handlers;
  handle_if(h, b, Bit::IRQ_BASIC_ARM_TIMER, domains::ARM, 0);
  handle_if(h, b, Bit::IRQ_BASIC_ARM_MBOX, domains::ARM, 1);
  handle_if(h, b, Bit::IRQ_BASIC_ARM_DOORBELL_0, domains::ARM, 2);
  handle_if(h, b, Bit::IRQ_BASIC_ARM_DOORBELL_1, domains::ARM, 3);
  handle_if(h, b, Bit::IRQ_BASIC_GPU_HALTED_0, domains::ARM, 4);
  handle_if(h, b, Bit::IRQ_BASIC_GPU_HALTED_1, domains::ARM, 5);
  handle_if(h, b, Bit::IRQ_BASIC_ILLEGAL_ACC_0, domains::ARM, 6);
  handle_if(h, b, Bit::IRQ_BASIC_ILLEGAL_ACC_1, domains::ARM, 7);

  handle_if(h, b, Bit::IRQ_BASIC_PEND_GPU_9, domains::PERIPHERAL, 9);
  handle_if(h, b, Bit::IRQ_BASIC_PEND_GPU_53, domains::PERIPHERAL, 53);
  handle_if(h, b, Bit::IRQ_BASIC_PEND_GPU_54, domains::PERIPHERAL, 54);
  handle_if(h, b, Bit::IRQ_BASIC_PEND_GPU_55, domains::PERIPHERAL, 55);
  handle_if(h, b, Bit::IRQ_BASIC_PEND_GPU_57, domains::PERIPHERAL, 57);

  if b & Bit::IRQ_PERIP_REG_1 > 0 {
    handle_if(h, p1, Bit::IRQ_1_SYSTEM_TIMER_1, domains::PERIPHERAL, 1);
    handle_if(h, p1, Bit::IRQ_1_SYSTEM_TIMER_3, domains::PERIPHERAL, 3);
    handle_if(h, p1, Bit::IRQ_1_USB, domains::PERIPHERAL, 9);
    handle_if(h, p1, Bit::IRQ_1_AUX, domains::PERIPHERAL, 29);
  }
  if b & Bit::IRQ_PERIP_REG_2 > 0 {
    handle_if(h, p2, Bit::IRQ_2_I2C_SPI_SLAVE, domains::PERIPHERAL, 43);
    handle_if(h, p2, Bit::IRQ_2_PWA_0, domains::PERIPHERAL, 45);
    handle_if(h, p2, Bit::IRQ_2_PWA_1, domains::PERIPHERAL, 46);
    handle_if(h, p2, Bit::IRQ_2_SMI, domains::PERIPHERAL, 48);
    handle_if(h, p2, Bit::IRQ_2_GPIO_0, domains::PERIPHERAL, 49);
    handle_if(h, p2, Bit::IRQ_2_GPIO_1, domains::PERIPHERAL, 50);
    handle_if(h, p2, Bit::IRQ_2_GPIO_2, domains::PERIPHERAL, 51);
    handle_if(h, p2, Bit::IRQ_2_GPIO_3, domains::PERIPHERAL, 52);
    handle_if(h, p2, Bit::IRQ_2_I2C, domains::PERIPHERAL, 53);
    handle_if(h, p2, Bit::IRQ_2_SPI, domains::PERIPHERAL, 54);
    handle_if(h, p2, Bit::IRQ_2_PCM, domains::PERIPHERAL, 55);
    handle_if(h, p2, Bit::IRQ_2_UART, domains::PERIPHERAL, 57);
  }
}

#[inline(always)]
fn handle_if(
  handlers: &[interrupt::HandlerMeta],
  val: u32,
  mask: u32,
  domain: interrupt::IrqDomain,
  number: u32,
) {
  if val & mask > 0 {
    handle(handlers, interrupt::IrqChannel { domain, number });
  }
}

fn handle(handlers: &[interrupt::HandlerMeta], channel: interrupt::IrqChannel) {
  // could've been better lmao
  for handler in handlers {
    if handler.channel == channel {
      return (handler.handler)();
    }
  }
  panic!(
    "Unhandled interrupt {:?}, #{}",
    channel.domain, channel.number
  );
}

#[inline]
fn valid_irq_peripheral_number(number: u32) -> bool {
  match number {
    1 | 3 | 9 | 29 | 43 | 45 | 46 | 48..=55 | 57 => true,
    _ => false,
  }
}
