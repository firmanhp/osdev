use crate::common::{self, stream};
use crate::metadata::cpu;
use crate::timer;

fn first_timer() {
  stream::println!("Timer woo woo! do it again!");
  timer::set_timer(5000 * 500, second_timer).expect("Not OK");
}

fn second_timer() {
  stream::println!("Second timer, yay!");
}

pub fn test_interrupt() -> ! {
  stream::println!("Executing in level {}", cpu::get_ring_level());
  stream::println!("Timer setup!");
  timer::set_timer(5000 * 500, first_timer).expect("Not OK");

  loop {
    common::synchronization::sleep(500_000_000);
    stream::println!("Timer wait...");
  }
}
