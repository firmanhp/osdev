use crate::io::uart;
use crate::meta::board_info;

pub fn test_board_info() -> ! {
  use crate::common::stream;
  // Test Clock mailbox
  uart::pl011_init();
  stream::println!("Test Board information");

  let board_info = board_info::raspi_board_info();
  stream::println!("===================================");
  stream::println!(
    "VideoCore Firmware rev: {}",
    &board_info.videocore_firmware_rev
  );
  stream::println!("Board type: {}", &board_info.board_type);
  stream::println!("Board model: {}", &board_info.board_model);
  stream::println!("MAC Address: {}", &board_info.board_mac_address);
  stream::println!("Serial: {:010x}", &board_info.board_serial);
  stream::println!("ARM");
  stream::println!(
    "\tBase address: {:#010x}",
    &board_info.arm_mem_base_address
  );
  stream::println!("\tSize: {} bytes", &board_info.arm_mem_size_bytes);
  stream::println!("VideoCore");
  stream::println!(
    "\tBase address: {:#010x}",
    &board_info.videocore_mem_base_address
  );
  stream::println!("\tSize: {} bytes", &board_info.videocore_mem_size_bytes);

  stream::println!("===================================");
  stream::println!("Test OK");
  loop {}
}
