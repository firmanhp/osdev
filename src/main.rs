#![cfg_attr(feature = "device", no_std)]
#![cfg_attr(feature = "device", no_main)]

mod asm;
mod boot;
mod common;
mod container;
mod diagnostic;
mod io;
mod meta;
mod synchronization;
mod tty;

use core::panic::PanicInfo;
use io::{gpio, mmio};
use tty::{TTY, TTYError};

/// Error type for kernel operations
#[derive(Debug)]
pub enum KernelError {
    InitializationError(&'static str),
    HardwareError(&'static str),
    IOError(&'static str),
    TTY(TTYError),
}

/// Implement conversion from `TTYError` to `KernelError`
impl From<TTYError> for KernelError {
    fn from(error: TTYError) -> Self {
        KernelError::TTY(error)
    }
}

/// Result type alias for kernel operations
type KernelResult<T> = Result<T, KernelError>;

/// Represents the state and operations of the kernel
struct Kernel {
    tty: TTY,
}

impl Kernel {
    /// Creates a new Kernel instance, initializing necessary components
    fn new() -> KernelResult<Self> {
        mmio::init();
        let tty = TTY::new();
        Ok(Self { tty })
    }

    /// Initializes the kernel and runs diagnostics
    fn initialize(&mut self) -> KernelResult<()> {
        self.tty.write("Kernel initialization started...\n")?;
        self.run_diagnostics()?;
        self.tty.write("Kernel initialization completed successfully.\n")?;
        Ok(())
    }

    /// Runs diagnostic tests on the board
    fn run_diagnostics(&mut self) -> KernelResult<()> {
        self.tty.write("Running board diagnostics...\n")?;
        
        // Running the diagnostics
        diagnostic::test_board_info(); // Note: Ensure this function does not return.
        Ok(())
    }
}

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn kernel_main() {
    match Kernel::new() {
        Ok(mut kernel) => {
            if let Err(e) = kernel.initialize() {
                handle_kernel_error(&mut kernel, &e);
            }
        }
        Err(e) => {
            let mut tty = TTY::new();
            let _ = tty.write(&format!("Failed to initialize Kernel: {:?}\n", e));
        }
    }
}

/// Logs kernel errors to TTY
fn handle_kernel_error(kernel: &mut Kernel, error: &KernelError) {
    let _ = kernel.tty.write(&format!("Kernel error: {:?}\n", error));
}

/// Structure representing the state during a panic
struct PanicState {
    tty: TTY,
    led_gpio: u64,
}

impl PanicState {
    const LED_GPIO: u64 = 1 << 27;
    const BLINK_DELAY: i32 = 250_000;

    /// Initializes a new PanicState
    fn new() -> Self {
        Self {
            tty: TTY::new(),
            led_gpio: Self::LED_GPIO,
        }
    }

    /// Handles panic by printing details and signaling an error state
    fn handle_panic(&mut self, info: &PanicInfo) {
        self.print_panic_info(info);
        self.configure_panic_led();
        self.blink_led_forever();
    }

    /// Prints panic information to TTY
    fn print_panic_info(&mut self, info: &PanicInfo) {
      let _ = self.tty.write("Panic occurred!\n");
      let _ = self.tty.write("PANIC: ");
  
      // Directly use the message without treating it as an Option
      let message = info.message();  // No need for Some check
      let _ = self.tty.write(&format!("{:?}\n", message));
  
      // Output location information, if available
      if let Some(location) = info.location() {
          let _ = self.tty.write(&format!(
              "Location: {}:{}\n",
              location.file(),
              location.line()
          ));
      }
  }
  
  

    /// Configures the LED to indicate a panic state
    fn configure_panic_led(&self) {
        gpio::set_function(self.led_gpio, gpio::Function::Output);
        gpio::set_pull_mode(self.led_gpio, gpio::PullMode::Disabled);
    }

    /// Blinks the LED indefinitely to signal panic
    fn blink_led_forever(&self) -> ! {
        loop {
            gpio::output_set(self.led_gpio);
            synchronization::sleep(Self::BLINK_DELAY);
            gpio::output_clear(self.led_gpio);
            synchronization::sleep(Self::BLINK_DELAY);
        }
    }
}

#[panic_handler]
#[cfg(feature = "device")]
fn on_panic(info: &PanicInfo) -> ! {
    let mut panic_state = PanicState::new();
    panic_state.handle_panic(info);
}
