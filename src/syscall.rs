#[cfg(test)]
use crate::io::mock_uart::{mock_getc as pl011_getc, mock_putc as pl011_putc};
#[cfg(not(test))]
use crate::io::uart::{pl011_getc, pl011_putc};

use core::result::Result;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SyscallID {
  UART_READ,
  UART_WRITE,
  UNKNOWN,
}

impl TryFrom<u32> for SyscallID {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(SyscallID::UART_READ),
      1 => Ok(SyscallID::UART_WRITE),
      _ => Ok(SyscallID::UNKNOWN),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum SyscallError {
  InvalidSyscall,
  WriteError,
  ReadError,
}

pub type SyscallResult = Result<u64, SyscallError>;
pub type SyscallFn = fn(u64, u64) -> SyscallResult;

pub struct SyscallTable {
  table: [Option<SyscallFn>; 2],
}

impl SyscallTable {
  pub fn new() -> Self {
    let mut table: [Option<SyscallFn>; 2] = [None; 2];
    table[SyscallID::UART_READ as usize] = Some(sys_uart_read);
    table[SyscallID::UART_WRITE as usize] = Some(sys_uart_write);
    SyscallTable { table }
  }

  pub fn dispatch(&self, id: SyscallID, arg1: u64, arg2: u64) -> SyscallResult {
    // Check if the ID is unknown or out of bounds and return InvalidSyscall error
    if id == SyscallID::UNKNOWN || id as usize >= self.table.len() {
      return Err(SyscallError::InvalidSyscall);
    }

    match self.table[id as usize] {
      Some(syscall_fn) => syscall_fn(arg1, arg2),
      None => Err(SyscallError::InvalidSyscall),
    }
  }
}

// Syscall handler for UART read
fn sys_uart_read(_: u64, _: u64) -> SyscallResult {
  let byte = pl011_getc();
  Ok(byte as u64)
}

// Syscall handler for UART write
fn sys_uart_write(byte: u64, _: u64) -> SyscallResult {
  pl011_putc(byte as u8);
  Ok(0)
}

#[cfg(test)]
#[path = "syscall_test.rs"]
mod syscall_test;
