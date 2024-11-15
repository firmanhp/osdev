use crate::common::stream;
use crate::metadata::cpu;
use crate::timer;

pub fn test_interrupt() -> ! {
  // TODO: Why EL3? move to EL1!!
  stream::println!("Executing in level {}", cpu::get_ring_level());
  stream::println!("Timer setup!");
  timer::set_timer(5000, || {
    stream::println!("Timer woo woo!");
  })
  .expect("Not OK");

  loop {}
}
