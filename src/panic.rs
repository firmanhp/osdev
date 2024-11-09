use crate::common::stream;
use crate::common::synchronization;
use crate::io::gpio;

#[panic_handler]
#[cfg(feature = "device")]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
  const GPIO: u64 = 1 << 27;

  stream::println!(
    "PANIC: {}",
    info.message().as_str().unwrap_or("No message")
  );

  if let Some(location) = info.location() {
    stream::println!("{}:{}", location.file(), location.line());
  }

  gpio::set_function(GPIO, gpio::Function::Output);
  gpio::set_pull_mode(GPIO, gpio::PullMode::Disabled);

  loop {
    gpio::output_set(GPIO);
    synchronization::sleep(250_000);
    gpio::output_clear(GPIO);
    synchronization::sleep(250_000);
  }
}
