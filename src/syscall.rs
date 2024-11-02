use crate::interrupt::{
  timer_handler, InterruptController, InterruptError, InterruptHandler,
  InterruptType,
};
#[cfg(test)]
use crate::io::mock_uart::{mock_getc as pl011_getc, mock_putc as pl011_putc};
#[cfg(not(test))]
use crate::io::uart::{pl011_getc, pl011_putc};
use core::result::Result;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SyscallID {
  UartRead,
  UartWrite,
  InterruptEnable,
  InterruptDisable,
  InterruptRegister,
  Invalid,
}

impl TryFrom<u32> for SyscallID {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(SyscallID::UartRead),
      1 => Ok(SyscallID::UartWrite),
      2 => Ok(SyscallID::InterruptEnable),
      3 => Ok(SyscallID::InterruptDisable),
      4 => Ok(SyscallID::InterruptRegister),
      _ => Ok(SyscallID::Invalid),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum SyscallError {
  InvalidSyscall,
  WriteError,
  ReadError,
  InterruptError,
}

impl From<InterruptError> for SyscallError {
  fn from(_: InterruptError) -> Self {
    SyscallError::InterruptError
  }
}

pub type SyscallResult = Result<u64, SyscallError>;
pub type SyscallFn = fn(u64, u64) -> SyscallResult;

pub struct SyscallTable {
  table: [Option<SyscallFn>; 5],
  interrupt_controller: InterruptController,
}

impl SyscallTable {
  pub fn new() -> Self {
    let mut table: [Option<SyscallFn>; 5] = [None; 5];
    table[SyscallID::UartRead as usize] = Some(sys_uart_read);
    table[SyscallID::UartWrite as usize] = Some(sys_uart_write);
    table[SyscallID::InterruptEnable as usize] = Some(sys_interrupt_enable);
    table[SyscallID::InterruptDisable as usize] = Some(sys_interrupt_disable);
    table[SyscallID::InterruptRegister as usize] = Some(sys_interrupt_register);

    SyscallTable {
      table,
      interrupt_controller: InterruptController::new(),
    }
  }

  pub fn dispatch(
    &mut self,
    id: SyscallID,
    arg1: u64,
    arg2: u64,
  ) -> SyscallResult {
    if id == SyscallID::Invalid || id as usize >= self.table.len() {
      return Err(SyscallError::InvalidSyscall);
    }

    match self.table[id as usize] {
      Some(syscall_fn) => match id {
        SyscallID::InterruptRegister => {
          self.handle_interrupt_registration(arg1, arg2)
        }
        SyscallID::InterruptEnable => {
          self.interrupt_controller.enable_interrupts();
          syscall_fn(arg1, arg2)
        }
        SyscallID::InterruptDisable => {
          self.interrupt_controller.disable_interrupts();
          syscall_fn(arg1, arg2)
        }
        _ => syscall_fn(arg1, arg2),
      },
      None => Err(SyscallError::InvalidSyscall),
    }
  }

  fn handle_interrupt_registration(
    &mut self,
    int_type: u64,
    handler_addr: u64,
  ) -> SyscallResult {
    let int_type_u32 =
      u32::try_from(int_type).map_err(|_| SyscallError::InterruptError)?;

    let interrupt_type = InterruptType::try_from(int_type_u32)
      .map_err(|_| SyscallError::InterruptError)?;

    // Safety: Converting the handler address to a function pointer
    let handler: InterruptHandler =
      unsafe { core::mem::transmute(handler_addr) };

    self
      .interrupt_controller
      .register_handler(interrupt_type, handler)
      .map(|_| 0)
      .map_err(Into::into)
  }

  pub fn handle_interrupt(
    &self,
    int_type: InterruptType,
  ) -> Result<(), SyscallError> {
    self
      .interrupt_controller
      .handle_interrupt(int_type)
      .map_err(Into::into)
  }
}

// Syscall handlers
fn sys_uart_read(_: u64, _: u64) -> SyscallResult {
  let byte = pl011_getc();
  Ok(byte as u64)
}

fn sys_uart_write(byte: u64, _: u64) -> SyscallResult {
  pl011_putc(byte as u8);
  Ok(0)
}

fn sys_interrupt_enable(_: u64, _: u64) -> SyscallResult {
  // Platform-specific interrupt enable would go here
  Ok(0)
}

fn sys_interrupt_disable(_: u64, _: u64) -> SyscallResult {
  // Platform-specific interrupt disable would go here
  Ok(0)
}

fn sys_interrupt_register(_int_type: u64, _handler_addr: u64) -> SyscallResult {
  // The actual registration is handled in handle_interrupt_registration
  Ok(0)
}

#[cfg(test)]
#[path = "syscall_test.rs"]
mod syscall_test;
