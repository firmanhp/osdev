#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::error::ErrorKind;
  use crate::common::stream::{set_out, OutputOps};
  use crate::diagnostic::board_info::{display_board_info, display_cpu_info};
  use crate::meta::board_info::MacAddress;
  use crate::meta::board_info::RaspiBoardType;
  use crate::meta::cpu_info::{CacheLevel, CacheType};
  use crate::meta::{board_info, cpu_info};
  use core::fmt::{self, Write};
  use lazy_static::lazy_static;
  use std::sync::Mutex;

  // Define a static buffer to capture printed output
  lazy_static! {
    static ref TEST_BUFFER: Mutex<TestBuffer> = Mutex::new(TestBuffer::new());
  }

  struct TestBuffer {
    buffer: Vec<u8>,
  }

  impl TestBuffer {
    fn new() -> Self {
      Self { buffer: Vec::new() }
    }

    fn contents(&self) -> String {
      String::from_utf8_lossy(&self.buffer).into_owned()
    }

    fn clear(&mut self) {
      self.buffer.clear();
    }
  }

  impl Write for TestBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
      self.buffer.extend_from_slice(s.as_bytes());
      Ok(())
    }
  }

  // Proxy function to write to the TEST_BUFFER
  fn write_to_test_buffer(s: &str) -> Result<(), ErrorKind> {
    let mut buffer = TEST_BUFFER.lock().unwrap();
    buffer.write_str(s).map_err(|_| ErrorKind::Uncategorized)
  }

  // Set up a test output environment and return a reference to TEST_BUFFER
  fn setup_test_output() -> &'static Mutex<TestBuffer> {
    let output_ops = OutputOps {
      write: write_to_test_buffer,
    };
    set_out(output_ops);

    let mut buffer = TEST_BUFFER.lock().unwrap();
    buffer.clear();
    &TEST_BUFFER
  }

  // Test for displaying board information
  #[test]
  fn test_display_board_info() {
    let test_buffer = setup_test_output();

    // Mock data for testing
    let mock_board_info = board_info::RaspiBoardInfo {
      videocore_firmware_rev: 12345,
      board_type: RaspiBoardType::Pi4,
      board_model: 4,
      board_mac_address: MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
      board_serial: 0x123456789ABCDEF,
      arm_mem_base_address: 0x1000,
      arm_mem_size_bytes: 1024 * 1024, // 1 MB
      videocore_mem_base_address: 0x2000,
      videocore_mem_size_bytes: 512 * 1024, // 512 KB
    };

    display_board_info(&mock_board_info);
    let output = test_buffer.lock().unwrap().contents();
    assert!(output.contains("Board Information:"));
    assert!(output.contains("VideoCore Firmware rev: 12345"));
    assert!(output.contains("Board type: Pi4"));
    assert!(output.contains("MAC Address: aa:bb:cc:dd:ee:ff"));
    assert!(output.contains("Serial: 123456789abcdef"));
    assert!(output.contains("ARM Memory:"));
    assert!(output.contains("Base address: 0x00001000"));
    assert!(output.contains("Size: 1048576 bytes"));
    assert!(output.contains("VideoCore Memory:"));
    assert!(output.contains("Base address: 0x00002000"));
    assert!(output.contains("Size: 524288 bytes"));
  }

  // Test for displaying cpu information
  #[test]
  fn test_display_cpu_info() {
    setup_test_output();

    // Mock CPU information data
    let mock_memory_model = cpu_info::MemoryModel {
      mmu_enabled: true,
      cache: [
        cpu_info::CacheInfo {
          level: CacheLevel::L1,
          cache_type: CacheType::Data,
          exists: true,
          line_size_bytes: 64,
          associativity: 4,
          num_sets: 128,
          total_size_bytes: 32768, // 32 KB
          write_alloc_supported: true,
          read_alloc_supported: true,
          write_back_supported: true,
          write_through_supported: false,
        },
        cpu_info::CacheInfo {
          level: CacheLevel::L2,
          cache_type: CacheType::Unified,
          exists: false,
          line_size_bytes: 0,
          associativity: 0,
          num_sets: 0,
          total_size_bytes: 0,
          write_alloc_supported: false,
          read_alloc_supported: false,
          write_back_supported: false,
          write_through_supported: false,
        },
        cpu_info::CacheInfo {
          level: CacheLevel::L3,
          cache_type: CacheType::Unified,
          exists: true,
          line_size_bytes: 64,
          associativity: 8,
          num_sets: 256,
          total_size_bytes: 2097152, // 2 MB
          write_alloc_supported: true,
          read_alloc_supported: true,
          write_back_supported: true,
          write_through_supported: false,
        },
        cpu_info::CacheInfo {
          level: CacheLevel::L3,
          cache_type: CacheType::Unified,
          exists: false,
          line_size_bytes: 0,
          associativity: 0,
          num_sets: 0,
          total_size_bytes: 0,
          write_alloc_supported: false,
          read_alloc_supported: false,
          write_back_supported: false,
          write_through_supported: false,
        },
      ],
    };

    // Run function and capture output
    display_cpu_info(&mock_memory_model);
    let output = TEST_BUFFER.lock().unwrap().contents();

    println!("Captured output:\n{}", output);

    // Assertions for CPU information
    assert!(output.contains("CPU Information:"));
    assert!(output.contains("Stage 1 MMU enabled: true"));
    assert!(output.contains("Cache Level L1 - data"));
    assert!(output.contains("Size Information:"));
    assert!(output.contains("Line size: 64 bytes"));
    assert!(output.contains("Associativity: 4"));
    assert!(output.contains("Number of sets: 128"));
    assert!(output.contains("Total size: 32768 bytes (32 KiB)"));
    assert!(output.contains("Write allocation: true"));
    assert!(output.contains("Read allocation: true"));
    assert!(output.contains("Write back: true"));
    assert!(output.contains("Write through: false"));

    // Assertions for non-existent cache levels
    assert!(output.contains("Cache Level L2 - unified"));
    assert!(output.contains("Cache does not exist"));

    // Assertions for L3 cache
    assert!(output.contains("Cache Level L3 - unified"));
    assert!(output.contains("Total size: 2097152 bytes (2048 KiB)"));
  }
}
