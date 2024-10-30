use crate::common::stream;
use crate::io::uart;
use crate::meta::board_info;
use crate::meta::cpu_info;

fn print_cache_info(cache: &cpu_info::CacheInfo) {
  stream::println!("Cache {} - {}", cache.level, cache.cache_type);
  if !cache.exists {
    stream::println!("Cache do not exist");
    return;
  }

  stream::println!("Line size (bytes): {}", cache.line_size_bytes);
  stream::println!("Associativity: {}", cache.associativity);
  stream::println!("Number of sets: {}", cache.num_sets);
  stream::println!(
    "Total size: {} bytes = {} KiB",
    cache.total_size_bytes,
    cache.total_size_bytes / 1024
  );
  stream::println!(
    "Write allocation supported: {}",
    cache.write_alloc_supported
  );
  stream::println!("Read allocation supported: {}", cache.read_alloc_supported);
  stream::println!("Write back supported: {}", cache.write_back_supported);
  stream::println!(
    "Write through supported: {}",
    cache.write_through_supported
  );
}

pub fn test_board_info() -> ! {
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

  stream::println!();
  stream::println!("===================================");
  stream::println!("Test CPU info");

  let memory_model = cpu_info::get_memory_model();
  /*
  ARMv8-A memory model:
  When the Stage 1 MMU is disabled:
  - All data accesses are Device_nGnRnE. We will explain this later in this guide.
  - All instruction fetches are treated as cacheable.
  - All addresses have read/write access and are executable.
  */
  stream::println!("Stage 1 MMU enabled: {}", memory_model.mmu_enabled);

  for cache_info in memory_model.cache {
    stream::println!();
    print_cache_info(&cache_info);
  }

  stream::println!();
  stream::println!("Test OK");
  loop {}
}
