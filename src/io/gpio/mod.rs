// src/io/gpio/mod.rs
//! GPIO (General Purpose Input/Output) module for configuring and controlling Raspberry Pi GPIO pins.
//!
//! # Overview
//! The GPIO module provides functions for setting the mode, function, and output state of GPIO pins on the Raspberry Pi.
//! Raspberry Pi supports up to 54 GPIO pins, each of which can be configured independently to various modes such as input,
//! output, or alternate functions for specific hardware peripherals like I2C or UART.
//!
//! # Usage Example
//! ```
//! use crate::io::gpio::{set_function, set_pull_mode, output_set, output_clear, Function, PullMode};
//!
//! // Set GPIO pins 5 and 10 as output
//! set_function(1 << 5 | 1 << 10, Function::Output);
//!
//! // Enable pull-up resistors on GPIO pins 6 and 11
//! set_pull_mode(1 << 6 | 1 << 11, PullMode::PullUp);
//!
//! // Set GPIO pin 5 to high
//! output_set(1 << 5);
//!
//! // Clear GPIO pin 5 to low
//! output_clear(1 << 5);
//! ```
//!
//! # Functionality
//! - **Pull Mode**: Set the pull-up/pull-down mode for GPIO pins to control their default state.
//! - **Function Selection**: Configure GPIO pins to act as input, output, or to be associated with alternate functions.
//! - **Output Control**: Set or clear GPIO output levels.
//!
//! # Implementation Details
//! GPIO pins are controlled by manipulating bits within various hardware registers. Each register manages a specific range of pins.

pub mod bcm2837_gpio;

static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

struct Ops {
  output_set: fn(u64),
  output_clear: fn(u64),
  set_pull_mode: fn(u64, PullMode),
  set_function: fn(u64, Function),
}

// Pull up/down control mode.
#[allow(dead_code)]
pub enum PullMode {
  // Off – disable pull-up/down
  Disabled,
  // Enable Pull Down control
  PullDown,
  // Enable Pull Up control
  PullUp,
}

/// GPIO Pin Functions
///
/// Different modes a GPIO pin can be set to.
#[allow(dead_code)]
pub enum Function {
  Input,
  Output,
  Func0,
  Func1,
  Func2,
  Func3,
  Func4,
  Func5,
}

/// Sets the pull-up/down mode for specified GPIO pins.
///
/// # Parameters
/// - `gpios`: A bitmask specifying the GPIO pins to configure.
/// - `mode`: The desired `PullMode` for the pins.
///
/// # Examples
/// ```
/// // Set pull-up on GPIO pins 0 and 1
/// set_pull_mode(1 << 0 | 1 << 1, PullMode::PullUp);
/// ```
pub fn set_pull_mode(gpios: u64, mode: PullMode) {
  unsafe {
    assert!(SET, "GPIO handler not set");
    (OPS.assume_init_ref().set_pull_mode)(gpios, mode);
  }
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
pub fn set_function(mut gpios: u64, function: Function) {
  unsafe {
    assert!(SET, "GPIO handler not set");
    (OPS.assume_init_ref().set_function)(gpios, function);
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
pub fn output_set(gpios: u64) {
  unsafe {
    assert!(SET, "GPIO handler not set");
    (OPS.assume_init_ref().output_set)(gpios);
  }
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
pub fn output_clear(gpios: u64) {
  unsafe {
    assert!(SET, "GPIO handler not set");
    (OPS.assume_init_ref().output_clear)(gpios);
  }
}

fn register_device(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}

// Initialize device driver
#[macro_export]
macro_rules! init {
  ($device_name:ident) => {{
    use crate::io::gpio::$device_name as device;
    device::device_init();
  }};
}

pub use init;
