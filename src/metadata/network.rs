static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

pub struct Ops {
  pub get_mac_address: fn() -> MacAddress,
}

pub struct MacAddress {
  // In network byte order
  pub data: [u8; 6],
}

impl MacAddress {
  pub fn new(data: [u8; 6]) -> Self {
    MacAddress { data }
  }
}

impl core::fmt::Display for MacAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
      self.data[0],
      self.data[1],
      self.data[2],
      self.data[3],
      self.data[4],
      self.data[5]
    )
  }
}

#[inline(always)]
pub fn get_mac_address() -> MacAddress {
  unsafe {
    assert!(SET, "No impl");
    (OPS.assume_init_ref().get_mac_address)()
  }
}

pub fn set_impl(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}
