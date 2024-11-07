#[cfg(test)]
#[cfg(feature = "host")]
mod tests {
  use super::*;
  use crate::io::uart;
  use crate::syscall::{SyscallError, SyscallID, SyscallTable};
  use std::convert::TryFrom;

  #[test]
  fn test_uart_read_syscall() {
    uart::uart_init!(mock);
    uart::mock::set_input("H");

    let syscall_table = SyscallTable::new();
    let result = syscall_table.dispatch(SyscallID::UartRead, 0, 0);

    assert_eq!(result, Ok('H' as u64));
  }

  #[test]
  fn test_uart_write_syscall() {
    uart::uart_init!(mock);

    let syscall_table = SyscallTable::new();
    let result = syscall_table.dispatch(SyscallID::UartWrite, 'A' as u64, 0);

    assert_eq!(result, Ok(0));
    assert_eq!(uart::mock::get_output(), vec!['A' as u8]);
  }

  #[test]
  fn test_invalid_syscall() {
    let syscall_table = SyscallTable::new();
    let invalid_syscall_id =
      SyscallID::try_from(99).unwrap_or(SyscallID::Invalid);
    let result = syscall_table.dispatch(invalid_syscall_id, 0, 0);

    assert_eq!(result, Err(SyscallError::InvalidSyscall));
  }
}
