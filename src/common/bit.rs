// Bit manipulation library

pub const fn bit<const POS: u8>() -> u32 {
  bit_range::<POS, POS>()
}

pub const fn bit_range<const MSB: u8, const LSB: u8>() -> u32 {
  if MSB < LSB {
    return bit_range::<LSB, MSB>();
  }
  ((1 << MSB) | ((1 << MSB) - 1)) & !((1 << LSB) - 1)
}

pub const fn bit_u64<const POS: u8>() -> u64 {
  bit_range_u64::<POS, POS>()
}

pub const fn bit_range_u64<const MSB: u8, const LSB: u8>() -> u64 {
  if MSB < LSB {
    return bit_range_u64::<LSB, MSB>();
  }
  ((1 << MSB) | ((1 << MSB) - 1)) & !((1 << LSB) - 1)
}

pub fn bit_of<const POS: u8>(val: u32) -> u32 {
  (val & bit::<POS>()) >> POS
}

pub fn bit_of_range<const MSB: u8, const LSB: u8>(val: u32) -> u32 {
  if MSB < LSB {
    return bit_of_range::<LSB, MSB>(val);
  }
  (val & bit_range::<MSB, LSB>()) >> LSB
}

pub fn bit_of_u64<const POS: u8>(val: u64) -> u64 {
  (val & bit_u64::<POS>()) >> POS
}

pub fn bit_of_range_u64<const MSB: u8, const LSB: u8>(val: u64) -> u64 {
  if MSB < LSB {
    return bit_of_range_u64::<LSB, MSB>(val);
  }
  (val & bit_range_u64::<MSB, LSB>()) >> LSB
}
