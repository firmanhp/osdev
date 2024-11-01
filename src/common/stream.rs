use crate::common::error;
use core::mem::MaybeUninit;

static mut OUT: MaybeUninit<OutputOps> = MaybeUninit::<OutputOps>::uninit();

pub struct OutputOps {
  pub write: fn(&str) -> core::result::Result<(), error::ErrorKind>,
}

impl core::fmt::Write for OutputOps {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    match (self.write)(s) {
      Ok(()) => Ok(()),
      // This type does not support transmission of an error other than
      // that an error occurred.
      Err(_kind) => Err(core::fmt::Error::default()),
    }
  }
}

pub fn out() -> &'static mut dyn core::fmt::Write {
  unsafe { OUT.assume_init_mut() }
}

pub fn set_out(streamer: OutputOps) {
  unsafe {
    OUT = MaybeUninit::<OutputOps>::new(streamer);
  }
}

#[macro_export]
macro_rules! print {
  ( $( $arg:expr ),* ) => {{
    use crate::common::stream::out;
    core::write!(out(), $($arg),*).expect("Print failed");
  }}
}

#[macro_export]
macro_rules! println {
  ( $( $arg:expr ),* ) => {{
    use crate::common::stream::out;
    core::writeln!(out(), $($arg),*).expect("Print failed");
  }}
}

pub use println;
