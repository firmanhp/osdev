use crate::io::clock;
use crate::io::clock::ClockId;
use crate::io::uart;

const CLOCK_IDS: [ClockId; 14] = [
  ClockId::Emmc,
  ClockId::Uart,
  ClockId::Arm,
  ClockId::Core,
  ClockId::V3d,
  ClockId::H264,
  ClockId::Isp,
  ClockId::Sdram,
  ClockId::Pixel,
  ClockId::Pwm,
  ClockId::Hevc,
  ClockId::Emmc2,
  ClockId::M2mc,
  ClockId::PixelBvb,
];

pub fn test_videocore_base_clock() -> ! {
  use crate::common::stream;
  // Test Clock mailbox
  uart::pl011_init();
  stream::println!("Test VideoCore base clock (via mailbox)");

  for clock_id in CLOCK_IDS {
    stream::println!("===================================");
    stream::println!("Clock: {}", clock_id);
    match clock::get_clock_info(clock_id) {
      Ok(info) => {
        stream::println!(
          "OK: {{ Id: {:#010X}, Exists: {}, Active: {}, Rate(Hz): {} }}",
          info.id as u32,
          info.exists,
          info.active,
          info.rate_hz
        );
      }
      Err(kind) => {
        stream::println!("FAIL: {}", kind);
      }
    }
    stream::println!();
  }

  stream::println!();
  stream::println!("Test done.");
  loop {}
}
