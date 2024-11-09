use arrayvec::ArrayString;
use arrayvec::ArrayVec;

use crate::container::arrayvec_extensions;

static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

pub struct Ops {
  pub get_board_info: fn() -> BoardInfo,
}

pub const ATTRIBUTE_KEY_CAP: usize = 16;
pub const ATTRIBUTE_VAL_CAP: usize = 32;
pub struct BoardAttribute {
  pub key: ArrayString<ATTRIBUTE_KEY_CAP>,
  pub value: ArrayString<ATTRIBUTE_VAL_CAP>,
}

impl BoardAttribute {
  fn new() -> BoardAttribute {
    BoardAttribute {
      key: ArrayString::<ATTRIBUTE_KEY_CAP>::new(),
      value: ArrayString::<ATTRIBUTE_VAL_CAP>::new(),
    }
  }

  pub fn of(
    key: &str,
    value: ArrayString<ATTRIBUTE_VAL_CAP>,
  ) -> BoardAttribute {
    BoardAttribute {
      key: arrayvec_extensions::capped_strings!(ATTRIBUTE_KEY_CAP, key),
      value,
    }
  }
}

pub const ATTRIBUTES_CAP: usize = 64;
pub struct BoardInfo {
  pub model: ArrayString<ATTRIBUTE_VAL_CAP>,
  pub serial: ArrayString<ATTRIBUTE_VAL_CAP>,
  pub attributes: ArrayVec<BoardAttribute, ATTRIBUTES_CAP>,
}

#[inline(always)]
pub fn get_board_info() -> BoardInfo {
  unsafe {
    assert!(SET, "No impl");
    (OPS.assume_init_ref().get_board_info)()
  }
}

pub fn set_impl(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}
