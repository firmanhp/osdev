// Bit manipulation library

pub const fn bit<const Pos: u8>() -> u32 {
  bit_range::<Pos, Pos>()
}

pub const fn bit_range<const Msb: u8, const Lsb: u8>() -> u32 {
  if Msb < Lsb {
    return bit_range::<Lsb, Msb>();
  }
  ((1 << Msb) | ((1 << Msb) - 1)) & !((1 << Lsb) - 1)
}

pub const fn bit_u64<const Pos: u8>() -> u64 {
  bit_range_u64::<Pos, Pos>()
}

pub const fn bit_range_u64<const Msb: u8, const Lsb: u8>() -> u64 {
  if Msb < Lsb {
    return bit_range_u64::<Lsb, Msb>();
  }
  ((1 << Msb) | ((1 << Msb) - 1)) & !((1 << Lsb) - 1)
}

pub fn bit_of<const Pos: u8>(val: u32) -> u32 {
  (val & bit::<Pos>()) >> Pos
}

pub fn bit_of_range<const Msb: u8, const Lsb: u8>(val: u32) -> u32 {
  if Msb < Lsb {
    return bit_of_range::<Lsb, Msb>(val);
  }
  (val & bit_range::<Msb, Lsb>()) >> Lsb
}

pub fn bit_of_u64<const Pos: u8>(val: u64) -> u64 {
  (val & bit_u64::<Pos>()) >> Pos
}

pub fn bit_of_range_u64<const Msb: u8, const Lsb: u8>(val: u64) -> u64 {
  if Msb < Lsb {
    return bit_of_range_u64::<Lsb, Msb>(val);
  }
  (val & bit_range_u64::<Msb, Lsb>()) >> Lsb
}
