use crate::io::gpio;
use crate::synchronization;

pub fn test_led_blink(gpio_ids: u64) {
  gpio::set_function(gpio_ids, gpio::Function::Output);

  loop {
    gpio::output_set(gpio_ids);
    synchronization::sleep(500_000);
    gpio::output_clear(gpio_ids);
    synchronization::sleep(500_000);
  }
}
