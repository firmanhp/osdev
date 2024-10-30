use super::*;

#[test]
fn test_bit() {
  use bit::bit;
  use bit::bit_u64;

  assert_eq!(bit::<0>(), 1 << 0);
  assert_eq!(bit::<15>(), 1 << 15);
  assert_eq!(bit::<31>(), 1 << 31);

  assert_eq!(bit_u64::<0>(), 1 << 0);
  assert_eq!(bit_u64::<15>(), 1 << 15);
  assert_eq!(bit_u64::<31>(), 1 << 31);
  assert_eq!(bit_u64::<32>(), 1 << 32);
  assert_eq!(bit_u64::<63>(), 1 << 63);
}

#[test]
fn test_bit_range() {
  use bit::bit_range;
  use bit::bit_range_u64;

  assert_eq!(bit_range::<0, 0>(), 1 << 0);
  assert_eq!(bit_range::<15, 15>(), 1 << 15);
  assert_eq!(bit_range::<31, 31>(), 1 << 31);

  assert_eq!(bit_range_u64::<0, 0>(), 1 << 0);
  assert_eq!(bit_range_u64::<15, 15>(), 1 << 15);
  assert_eq!(bit_range_u64::<31, 31>(), 1 << 31);
  assert_eq!(bit_range_u64::<32, 32>(), 1 << 32);
  assert_eq!(bit_range_u64::<63, 63>(), 1 << 63);

  assert_eq!(bit_range::<31, 0>(), 0xFFFF_FFFF);
  assert_eq!(bit_range::<0, 31>(), 0xFFFF_FFFF);
  assert_eq!(bit_range::<15, 4>(), 0xFFF0);
  assert_eq!(bit_range::<4, 15>(), 0xFFF0);
  assert_eq!(bit_range::<7, 3>(), 0b1111_1000);
  assert_eq!(bit_range::<3, 7>(), 0b1111_1000);

  assert_eq!(bit_range_u64::<31, 0>(), 0xFFFF_FFFF);
  assert_eq!(bit_range_u64::<0, 31>(), 0xFFFF_FFFF);
  assert_eq!(bit_range_u64::<15, 4>(), 0xFFF0);
  assert_eq!(bit_range_u64::<4, 15>(), 0xFFF0);
  assert_eq!(bit_range_u64::<7, 3>(), 0b1111_1000);
  assert_eq!(bit_range_u64::<3, 7>(), 0b1111_1000);

  assert_eq!(bit_range_u64::<63, 0>(), 0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(bit_range_u64::<0, 63>(), 0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(bit_range_u64::<63, 32>(), 0xFFFF_FFFF_0000_0000);
  assert_eq!(bit_range_u64::<63, 32>(), 0xFFFF_FFFF_0000_0000);
  assert_eq!(bit_range_u64::<47, 16>(), 0x0000_FFFF_FFFF_0000);
  assert_eq!(bit_range_u64::<47, 16>(), 0x0000_FFFF_FFFF_0000);
}
