use crate::common::stream;
use crate::interrupt::bcm2837_interrupt;
use crate::metadata::{board, cpu};
use crate::timer;

/// Prints detailed information about a given cache level, including size and capabilities.
fn display_cache_info(cache: &cpu::CacheInfo) {
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
    cache.total_size_bytes / 1024 as u32
  );

  stream::println!("Cache Capabilities:");
  stream::println!("\tWrite allocation: {}", cache.write_alloc_supported);
  stream::println!("\tRead allocation: {}", cache.read_alloc_supported);
  stream::println!("\tWrite back: {}", cache.write_back_supported);
  stream::println!("\tWrite through: {}", cache.write_through_supported);
}

/// Displays information about the Raspberry Pi board, including firmware, type, and memory layout.
fn display_board_info(board_info: &board::BoardInfo) {
  stream::println!("===================================");
  stream::println!("Board Information:");
  stream::println!("Model: {}", board_info.model);
  stream::println!("Serial: {}", board_info.serial);
  stream::println!();
  stream::println!("Attributes:");
  for attribute in &board_info.attributes {
    stream::println!("{}: {}", attribute.key, attribute.value);
  }
}

/**
 * Prints the CPU information, focusing on the MMU (Memory Management Unit) status.
 *
 * ## MMU State Information:
 *
 * **ARMv8-A Memory Model (when Stage 1 MMU is disabled)**:
 *
 * - **Data Accesses**: All data accesses are treated as `Device_nGnRnE` (Device Non-Gathering,
 *   Non-Reordering, Non-Early write acknowledgment). This restricts caching and enforces
 *   strict ordering for memory-mapped devices to ensure accurate I/O operations.
 * - **Instruction Fetches**: Treated as cacheable, allowing fast access to instructions but
 *   without strict protection control.
 * - **Access Permissions**: All addresses have full read/write permissions and are executable.
 *   This disables address-based permissions, making memory access permissive.
 *
 * The MMU enables virtual memory mappings, providing memory protection, address translation,
 * and isolation. When enabled, this allows for more efficient use of memory resources
 * and greater control over process isolation and memory security.
 *
 * - `Stage 1 MMU enabled`: Indicates if the first stage of the MMU is active.
 */
fn display_cpu_info(memory_model: &cpu::MemoryModel) {
  stream::println!("===================================");
  stream::println!("CPU Information:");

  stream::println!("Stage 1 MMU enabled: {}", memory_model.mmu_enabled);
  if !memory_model.mmu_enabled {
    stream::println!(
      "Note: ARMv8-A memory model when Stage 1 MMU is disabled:"
    );
    stream::println!(
      "\t- All data accesses are Device_nGnRnE (non-cacheable, non-shareable)."
    );
    stream::println!("\t- All instruction fetches are treated as cacheable.");
    stream::println!(
      "\t- All addresses have read/write access and are executable."
    );
  }

  for cache_info in &memory_model.cache {
    stream::println!();
    display_cache_info(cache_info);
  }
}

/// Entry point for the board information test. Initializes UART for serial
/// output, retrieves board and CPU information, and displays both through the
/// UART stream.
///
/// This function runs in a continuous loop after output to indicate the
/// completion of the test.
pub fn test_board_info() -> ! {
  stream::println!("Starting Board Information Test");

  let board_info = board::get_board_info();
  display_board_info(&board_info);

  let memory_model = cpu::get_memory_model();
  display_cpu_info(&memory_model);

  stream::println!(
    "ARM IRQ domain: {:?}",
    bcm2837_interrupt::domains::ARM.get()
  );
  stream::println!(
    "PERIPHERAL IRQ domain: {:?}",
    bcm2837_interrupt::domains::PERIPHERAL.get()
  );
  stream::println!("\nTest completed successfully");

  loop {}
}
