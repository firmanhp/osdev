use crate::io::uart;
use core::result::Result;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SyscallID {
  UartRead,
  UartWrite,
  Invalid,
}

impl TryFrom<u32> for SyscallID {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(SyscallID::UartRead),
      1 => Ok(SyscallID::UartWrite),
      _ => Ok(SyscallID::Invalid),
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
    table[SyscallID::UartRead as usize] = Some(sys_uart_read);
    table[SyscallID::UartWrite as usize] = Some(sys_uart_write);
    SyscallTable { table }
  }

  pub fn dispatch(&self, id: SyscallID, arg1: u64, arg2: u64) -> SyscallResult {
    // Check if the ID is invalid or out of bounds and return InvalidSyscall error
    if id == SyscallID::Invalid || id as usize >= self.table.len() {
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
  let byte = uart::getc();
  Ok(byte as u64)
}

// Syscall handler for UART write
fn sys_uart_write(byte: u64, _: u64) -> SyscallResult {
  uart::putc(byte as u8);
  Ok(0)
}

#[cfg(test)]
#[path = "syscall_test.rs"]
mod syscall_test;
