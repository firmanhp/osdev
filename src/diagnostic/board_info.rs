use crate::common::stream;
use crate::io::uart;
use crate::meta::{board_info, cpu_info};

const BYTES_PER_KIB: usize = 1024;

/// Represents and displays a memory region
struct MemoryRegion {
  base_address: usize,
  size_bytes: usize,
}

impl MemoryRegion {
  fn new(base_address: u32, size_bytes: u32) -> Self {
    Self {
      base_address: base_address as usize,
      size_bytes: size_bytes as usize,
    }
  }

  fn display(&self, region_name: &str) {
    stream::println!("{}:", region_name);
    stream::println!("\tBase address: {:#010x}", self.base_address);
    stream::println!("\tSize: {} bytes", self.size_bytes);
  }
}

/// Handles the display of cache information
fn display_cache_info(cache: &cpu_info::CacheInfo) {
  stream::println!("Cache Level {} - {}", cache.level, cache.cache_type);

  if !cache.exists {
    stream::println!("Cache does not exist");
    return;
  }

  stream::println!("Size Information:");
  stream::println!("\tLine size: {} bytes", cache.line_size_bytes);
  stream::println!("\tAssociativity: {}", cache.associativity);
  stream::println!("\tNumber of sets: {}", cache.num_sets);
  stream::println!(
    "\tTotal size: {} bytes ({} KiB)",
    cache.total_size_bytes,
    cache.total_size_bytes / BYTES_PER_KIB as u32
  );

  stream::println!("Cache Capabilities:");
  stream::println!("\tWrite allocation: {}", cache.write_alloc_supported);
  stream::println!("\tRead allocation: {}", cache.read_alloc_supported);
  stream::println!("\tWrite back: {}", cache.write_back_supported);
  stream::println!("\tWrite through: {}", cache.write_through_supported);
}

/// Displays general board information including memory regions
fn display_board_info(board_info: &board_info::RaspiBoardInfo) {
  stream::println!("===================================");
  stream::println!("Board Information:");
  stream::println!(
    "VideoCore Firmware rev: {}",
    board_info.videocore_firmware_rev
  );
  stream::println!("Board type: {}", board_info.board_type);
  stream::println!("Board model: {}", board_info.board_model);
  stream::println!("MAC Address: {}", board_info.board_mac_address);
  stream::println!("Serial: {:010x}", board_info.board_serial);

  // Display ARM and VideoCore memory regions
  MemoryRegion::new(
    board_info.arm_mem_base_address,
    board_info.arm_mem_size_bytes,
  )
  .display("ARM Memory");
  MemoryRegion::new(
    board_info.videocore_mem_base_address,
    board_info.videocore_mem_size_bytes,
  )
  .display("VideoCore Memory");
}

/// Displays CPU information including MMU and cache details
fn display_cpu_info(memory_model: &cpu_info::MemoryModel) {
  stream::println!("===================================");
  stream::println!("CPU Information:");

  /*
  ARMv8-A memory model:
  When the Stage 1 MMU is disabled:
  - All data accesses are Device_nGnRnE. We will explain this later in this guide.
  - All instruction fetches are treated as cacheable.
  - All addresses have read/write access and are executable.
  */
  stream::println!("Stage 1 MMU enabled: {}", memory_model.mmu_enabled);
  if !memory_model.mmu_enabled {
    stream::println!("Note: When MMU is disabled:");
    stream::println!("\t- All data accesses are Device_nGnRnE");
    stream::println!("\t- All instruction fetches are treated as cacheable");
    stream::println!(
      "\t- All addresses have read/write access and are executable"
    );
  }

  for cache_info in &memory_model.cache {
    stream::println!();
    display_cache_info(cache_info);
  }
}

/// Entry point for displaying board and CPU information
pub fn test_board_info() -> ! {
  uart::pl011_init();
  stream::println!("Starting Board Information Test");

  let board_info = board_info::raspi_board_info();
  display_board_info(&board_info);

  let memory_model = cpu_info::get_memory_model();
  display_cpu_info(&memory_model);

  stream::println!("\nTest completed successfully");
  loop {}
}

#[cfg(test)]
#[path = "board_info_test.rs"]
mod board_info_test;
