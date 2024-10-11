use core::hint::black_box;

// Loop <delay> times in a way that the compiler won't optimize away
pub fn sleep(count: i32) {
  black_box((|mut cnt: i32| while cnt > 0 { cnt -= 1; } ) (count));
}
