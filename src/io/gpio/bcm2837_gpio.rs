use crate::common::synchronization;
use crate::io::gpio;
use crate::io::mmio;

/// Total number of GPIO pins available on BCM2837.
const NUM_GPIOS: u8 = 54;

/// GPIO Register Addresses and Banks
///
/// This struct holds base addresses for the BCM2837's GPIO registers, enabling
/// memory-mapped access to GPIO configurations, function settings, and output control.
struct Reg;

#[allow(dead_code)]
impl Reg {
  const BASE: u64 = 0x0020_00_00;
  const GPFSEL0: u64 = Reg::BASE + 0x00; // GPIO Function Select 0
  const GPFSEL1: u64 = Reg::BASE + 0x04; // GPIO Function Select 1
  const GPFSEL2: u64 = Reg::BASE + 0x08; // GPIO Function Select 2
  const GPFSEL3: u64 = Reg::BASE + 0x0C; // GPIO Function Select 3
  const GPFSEL4: u64 = Reg::BASE + 0x10; // GPIO Function Select 4
  const GPFSEL5: u64 = Reg::BASE + 0x14; // GPIO Function Select 5
                                         // Each register represents floor(32bit / 3bit) = 10 GPIOs
                                         // bit 30-31 is reserved
  const GPFSEL_BANK: [u64; 6] = [
    Reg::GPFSEL0,
    Reg::GPFSEL1,
    Reg::GPFSEL2,
    Reg::GPFSEL3,
    Reg::GPFSEL4,
    Reg::GPFSEL5,
  ];

  const GPSET0: u64 = Reg::BASE + 0x1C; // GPIO Pin Output Set 0
  const GPSET1: u64 = Reg::BASE + 0x20; // GPIO Pin Output Set 1
  const GPCLR0: u64 = Reg::BASE + 0x28; // GPIO Pin Output Clear 0
  const GPCLR1: u64 = Reg::BASE + 0x2C; // GPIO Pin Output Clear 1

  const GPPUD: u64 = Reg::BASE + 0x94; // GPIO Pin Pull-up/down Enable
  const GPPUDCLK0: u64 = Reg::BASE + 0x98; // GPIO Pin Pull-up/down Enable Clock 0
  const GPPUDCLK1: u64 = Reg::BASE + 0x9C; // GPIO Pin Pull-up/down Enable Clock 1
}

fn set_pull_mode(mut gpios: u64, mode: gpio::PullMode) {
  // Make sure only 0..53 are set
  gpios = gpios & ((1 << NUM_GPIOS) - 1);
  let gpios_0: u32 = gpios as u32;
  let gpios_1: u32 = (gpios >> 32) as u32;

  // Write to GPPUD to set the required control signal
  let mode_val = match mode {
    gpio::PullMode::Disabled => 0x00,
    gpio::PullMode::PullDown => 0x01,
    gpio::PullMode::PullUp => 0x10,
  };
  mmio::write(Reg::GPPUD, mode_val);
  // Wait 150 cycles – this provides the required set-up time for the control signal
  synchronization::sleep(150);
  // Write to GPPUDCLK0/1 to clock the control signal into the GPIO pads
  // you wish to modify – NOTE only the pads which receive a clock will be
  // modified, all others will retain their previous state
  mmio::write(Reg::GPPUDCLK0, gpios_0);
  mmio::write(Reg::GPPUDCLK1, gpios_1);
  // Wait 150 cycles – this provides the required hold time for the control signal
  synchronization::sleep(150);

  // Write to GPPUD to remove the control signal
  mmio::write(Reg::GPPUD, 0x00);
  // Write to GPPUDCLK0/1 to remove the clock
  mmio::write(Reg::GPPUDCLK0, 0x00);
  mmio::write(Reg::GPPUDCLK1, 0x00);
}

/// Configures the function of the specified GPIO pins.
///
/// # Parameters
/// - `gpios`: A bitmask specifying the GPIO pins to configure.
/// - `function`: The `Function` to set for the specified pins.
///
/// # Examples
/// ```
/// // Set GPIO pins 5 and 10 to output
/// set_function(1 << 5 | 1 << 10, Function::Output);
/// ```
fn set_function(mut gpios: u64, function: gpio::Function) {
  assert!(NUM_GPIOS as usize <= 10 * Reg::GPFSEL_BANK.len());

  // Make sure only 0..53 are set
  gpios = gpios & ((1 << NUM_GPIOS) - 1);
  let function_bits: u8 = {
    match function {
      gpio::Function::Input => 0b000,
      gpio::Function::Output => 0b001,
      gpio::Function::Func0 => 0b100,
      gpio::Function::Func1 => 0b101,
      gpio::Function::Func2 => 0b110,
      gpio::Function::Func3 => 0b111,
      gpio::Function::Func4 => 0b011,
      gpio::Function::Func5 => 0b010,
    }
  };

  let mut gpfsel_val: [u32; Reg::GPFSEL_BANK.len()] =
    [0; Reg::GPFSEL_BANK.len()];
  for (idx, reg) in Reg::GPFSEL_BANK.iter().enumerate() {
    gpfsel_val[idx] = mmio::read(*reg);
  }

  for i in 0..(NUM_GPIOS - 1) {
    if gpios & (1 << i) != 0 {
      // Where to place the function bit
      let shift = (i % 10) * 3;
      let bank_idx = i / 10;
      // zero out the bits first
      gpfsel_val[bank_idx as usize] &= !((0b111 << shift) as u32);
      gpfsel_val[bank_idx as usize] |= (function_bits as u32) << shift;
    }
  }

  // Write updated values back to GPFSEL registers.
  for (reg, val) in core::iter::zip(Reg::GPFSEL_BANK, gpfsel_val) {
    mmio::write(reg, val);
  }
}

/// Sets the output of gpio in which position the bit is set.
/// For example, output_set(1 << 5 | 1 << 10) sets GPIO 5, and 10.
///
/// # Parameters
/// - `gpios`: A bitmask specifying the GPIO pins to set high.
///
/// # Examples
/// ```
/// // Set GPIO pins 5 and 10 to high
/// output_set(1 << 5 | 1 << 10);
/// ```
fn output_set(mut gpios: u64) {
  // Make sure only 0..53 is set
  gpios = gpios & ((1 << NUM_GPIOS) - 1);
  let gpios_0: u32 = gpios as u32;
  let gpios_1: u32 = (gpios >> 32) as u32;
  mmio::write(Reg::GPSET0, gpios_0);
  mmio::write(Reg::GPSET1, gpios_1);
}

/// Clears (sets to low) the specified GPIO pins.
// For example, output_set(1 << 5 | 1 << 10) clears GPIO 5, and 10.
///
/// # Parameters
/// - `gpios`: A bitmask specifying the GPIO pins to clear.
///
/// # Examples
/// ```
/// // Clear GPIO pins 5 and 10
/// output_clear(1 << 5 | 1 << 10);
/// ```
fn output_clear(mut gpios: u64) {
  // Make sure only 0..53 is set
  gpios = gpios & ((1 << NUM_GPIOS) - 1);
  let gpios_0: u32 = gpios as u32;
  let gpios_1: u32 = (gpios >> 32) as u32;
  mmio::write(Reg::GPCLR0, gpios_0);
  mmio::write(Reg::GPCLR1, gpios_1);
}

pub fn device_init() {
  gpio::register_device(gpio::Ops {
    output_set,
    output_clear,
    set_pull_mode,
    set_function,
  });
}
