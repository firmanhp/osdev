#[cfg(test)]
mod tests {
  use super::*;
  use crate::interrupt::{
    timer_handler, uart_handler, InterruptController, InterruptError,
    InterruptHandler, InterruptType,
  };

  #[test]
  fn test_interrupt_registration() {
    let mut controller = InterruptController::new();

    // Test successful registration
    assert!(controller
      .register_handler(InterruptType::Timer, timer_handler)
      .is_ok());

    // Test duplicate registration
    assert_eq!(
      controller.register_handler(InterruptType::Timer, timer_handler),
      Err(InterruptError::HandlerExists)
    );

    // Test unknown interrupt
    assert_eq!(
      controller.register_handler(InterruptType::Unknown, timer_handler),
      Err(InterruptError::UnknownInterrupt)
    );
  }

  #[test]
  fn test_interrupt_handling() {
    let mut controller = InterruptController::new();

    // Register handlers
    controller
      .register_handler(InterruptType::Timer, timer_handler)
      .unwrap();
    controller
      .register_handler(InterruptType::Uart, uart_handler)
      .unwrap();

    // Test with interrupts disabled
    assert!(controller.handle_interrupt(InterruptType::Timer).is_ok());

    // Enable interrupts and test
    controller.enable_interrupts();
    assert!(controller.handle_interrupt(InterruptType::Timer).is_ok());
    assert!(controller.handle_interrupt(InterruptType::Uart).is_ok());

    // Test unknown interrupt
    assert_eq!(
      controller.handle_interrupt(InterruptType::Unknown),
      Err(InterruptError::UnknownInterrupt)
    );
  }
}
