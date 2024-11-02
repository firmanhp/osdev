use core::result::Result;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InterruptType {
  Timer,
  Uart,
  Unknown,
}

impl TryFrom<u32> for InterruptType {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(InterruptType::Timer),
      1 => Ok(InterruptType::Uart),
      _ => Ok(InterruptType::Unknown),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum InterruptError {
  InvalidHandler,
  HandlerExists,
  UnknownInterrupt,
}

pub type InterruptResult = Result<(), InterruptError>;
pub type InterruptHandler = fn() -> InterruptResult;

pub struct InterruptController {
  handlers: [Option<InterruptHandler>; 2],
  interrupts_enabled: AtomicBool,
}

impl InterruptController {
  pub fn new() -> Self {
    InterruptController {
      handlers: [None; 2],
      interrupts_enabled: AtomicBool::new(false),
    }
  }

  /// Register a handler for a specific interrupt type
  pub fn register_handler(
    &mut self,
    int_type: InterruptType,
    handler: InterruptHandler,
  ) -> InterruptResult {
    if int_type == InterruptType::Unknown {
      return Err(InterruptError::UnknownInterrupt);
    }

    let idx = int_type as usize;
    if idx >= self.handlers.len() {
      return Err(InterruptError::InvalidHandler);
    }

    if self.handlers[idx].is_some() {
      return Err(InterruptError::HandlerExists);
    }

    self.handlers[idx] = Some(handler);
    Ok(())
  }

  /// Handle an interrupt of the specified type
  pub fn handle_interrupt(&self, int_type: InterruptType) -> InterruptResult {
    if !self.interrupts_enabled.load(Ordering::SeqCst) {
      return Ok(());
    }

    if int_type == InterruptType::Unknown {
      return Err(InterruptError::UnknownInterrupt);
    }

    let idx = int_type as usize;
    if idx >= self.handlers.len() {
      return Err(InterruptError::InvalidHandler);
    }

    match self.handlers[idx] {
      Some(handler) => handler(),
      None => Ok(()),
    }
  }

  /// Enable interrupts globally
  pub fn enable_interrupts(&self) {
    self.interrupts_enabled.store(true, Ordering::SeqCst);
  }

  /// Disable interrupts globally
  pub fn disable_interrupts(&self) {
    self.interrupts_enabled.store(false, Ordering::SeqCst);
  }

  /// Check if interrupts are enabled
  pub fn are_interrupts_enabled(&self) -> bool {
    self.interrupts_enabled.load(Ordering::SeqCst)
  }
}

// Example interrupt handlers
pub fn timer_handler() -> InterruptResult {
  // Handle timer interrupt
  Ok(())
}

pub fn uart_handler() -> InterruptResult {
  // Handle UART interrupt
  Ok(())
}

#[cfg(test)]
#[path = "interrupt_test.rs"]
mod interrupt_test;
