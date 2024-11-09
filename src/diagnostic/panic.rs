use crate::common::stream;

pub fn test_panic() -> ! {
  stream::println!("Testing panic");
  panic!("Testing");
}
