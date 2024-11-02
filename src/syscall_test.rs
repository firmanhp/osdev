#[cfg(test)]
mod tests {
  use super::*;
  use crate::io::mock_uart::{
    get_output, mock_getc as pl011_getc, mock_putc as pl011_putc,
    set_mock_input, setup,
  };
  use crate::syscall::InterruptError;
  use crate::syscall::{SyscallError, SyscallID, SyscallTable};

  #[test]
  fn test_syscall_dispatch() {
    setup();
    let mut syscall_table = SyscallTable::new();

    // Test UART syscalls with mocked functions
    // The mock functions are already configured through the cfg(test) feature
    assert!(syscall_table.dispatch(SyscallID::UartWrite, 65, 0).is_ok()); // Write 'A'
    assert!(syscall_table.dispatch(SyscallID::UartRead, 0, 0).is_ok());

    // Test interrupt syscalls
    assert!(syscall_table
      .dispatch(SyscallID::InterruptEnable, 0, 0)
      .is_ok());
    assert!(syscall_table
      .dispatch(SyscallID::InterruptDisable, 0, 0)
      .is_ok());
  }

  #[test]
  fn test_invalid_syscall() {
    let mut syscall_table = SyscallTable::new();
    assert_eq!(
      syscall_table.dispatch(SyscallID::Invalid, 0, 0),
      Err(SyscallError::InvalidSyscall)
    );
  }

  #[test]
  fn test_interrupt_registration() {
    let mut syscall_table = SyscallTable::new();

    // Create a dummy handler function for testing
    fn test_handler() -> Result<(), InterruptError> {
      Ok(())
    }

    // Register the test handler
    let handler_addr = test_handler as u64;
    assert!(syscall_table
      .dispatch(SyscallID::InterruptRegister, 0, handler_addr)
      .is_ok());
  }

  #[test]
  fn test_interrupt_registration_invalid_type() {
    let mut syscall_table = SyscallTable::new();

    fn test_handler() -> Result<(), InterruptError> {
      Ok(())
    }

    // Try to register with invalid interrupt type
    let handler_addr = test_handler as u64;
    assert_eq!(
      syscall_table.dispatch(SyscallID::InterruptRegister, 999, handler_addr),
      Err(SyscallError::InterruptError)
    );
  }
}
