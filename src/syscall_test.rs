#[cfg(test)]
mod tests {
  use super::*;
  use crate::io::mock_uart::{get_output, set_mock_input, setup};
  use crate::syscall::{SyscallError, SyscallID, SyscallTable};
  use std::convert::TryFrom;

  #[test]
  fn test_uart_read_syscall() {
    setup();
    set_mock_input("H");

    let syscall_table = SyscallTable::new();
    let result = syscall_table.dispatch(SyscallID::UART_READ, 0, 0);

    assert_eq!(result, Ok('H' as u64));
  }

  #[test]
  fn test_uart_write_syscall() {
    setup();

    let syscall_table = SyscallTable::new();
    let result = syscall_table.dispatch(SyscallID::UART_WRITE, 'A' as u64, 0);

    assert_eq!(result, Ok(0));
    assert_eq!(get_output(), vec!['A' as u8]);
  }

  #[test]
  fn test_invalid_syscall() {
    let syscall_table = SyscallTable::new();
    let invalid_syscall_id =
      SyscallID::try_from(99).unwrap_or(SyscallID::UNKNOWN);
    let result = syscall_table.dispatch(invalid_syscall_id, 0, 0);

    assert_eq!(result, Err(SyscallError::InvalidSyscall));
  }
}
