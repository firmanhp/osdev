use arrayvec::ArrayString;

// Support writing until the string is full, which will be no-op afterwards.
pub trait CappedWrite {
  fn write_str_capped(&mut self, s: &str) -> core::fmt::Result;
}

impl<const CAP: usize> CappedWrite for ArrayString<CAP> {
  fn write_str_capped(&mut self, s: &str) -> core::fmt::Result {
    let final_len = core::cmp::min(s.len(), self.capacity() - self.len());
    self.try_push_str(&s[..final_len]);
    Ok(())
  }
}

impl<'a> core::fmt::Write for (dyn CappedWrite + 'a) {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.write_str_capped(s)
  }
}

pub fn as_capped_write<'a, const CAP: usize>(
  s: &'a mut ArrayString<CAP>,
) -> &'a mut dyn CappedWrite {
  s as &mut dyn CappedWrite
}

pub fn make_str_capped<const CAP: usize>(s: &str) -> ArrayString<CAP> {
  use core::fmt::Write;
  let mut ret = ArrayString::<CAP>::new();
  let _ = core::write!(as_capped_write(&mut ret), "{}", s);
  ret
}

#[macro_export]
macro_rules! capped_strings {
  ($cap:expr, $( $s: expr ),*) => {{
      use core::fmt::Write;
      use arrayvec::ArrayString;
      use crate::container::arrayvec_extensions::as_capped_write;
      let mut ret = ArrayString::<$cap>::new();
      $(let _ = core::write!(as_capped_write(&mut ret), "{}", $s); )*
      ret
  }}
}

#[macro_export]
macro_rules! capped_format {
  ($cap:expr, $( $arg:expr ),* ) => {{
    use core::fmt::Write;
    use arrayvec::ArrayString;
    use crate::container::arrayvec_extensions::as_capped_write;
    let mut ret = ArrayString::<$cap>::new();
    let _ = core::write!(as_capped_write(&mut ret), $($arg),*);
    ret
  }}
}

pub use capped_format;
pub use capped_strings;
