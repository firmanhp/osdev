use crate::arch::arm64::asm;
use crate::common::bit;
use crate::common::bit::bit_of;
use crate::common::bit::bit_of_range;
use crate::metadata::cpu::CacheInfo;
use crate::metadata::cpu::CacheLevel;
use crate::metadata::cpu::CacheType;
use crate::metadata::cpu::MemoryModel;

struct Bit {}
impl Bit {
  // Or unified
  const CSSELR_EL1_DATA_UNIFIED: u8 = 0b0;
  const CSSELR_EL1_INSTR: u8 = 0b1;
  const CSSELR_EL1_L1: u8 = 0b000 << 1;
  const CSSELR_EL1_L2: u8 = 0b001 << 1;
  const CSSELR_EL1_L3: u8 = 0b010 << 1;
}

// Internal struct for SCTRL_EL1
// Can add more once they are needed
// https://developer.arm.com/documentation/ddi0595/2021-06/AArch64-Registers/SCTLR-EL1--System-Control-Register--EL1-
struct SCTRL {
  // M, bit[0]
  m: bool,
}

fn read_cache_info(
  cache_level: CacheLevel,
  cache_type: CacheType,
) -> CacheInfo {
  // https://developer.arm.com/documentation/100442/0100/register-descriptions/aarch64-system-registers/csselr-el1--cache-size-selection-register--el1
  let csselr_el1: u64 = {
    let data_bit: u8 = (cache_type == CacheType::Instruction) as u8;
    match cache_level {
      CacheLevel::L1 => Bit::CSSELR_EL1_L1 | data_bit,
      CacheLevel::L2 => Bit::CSSELR_EL1_L2 | data_bit,
      CacheLevel::L3 => Bit::CSSELR_EL1_L3 | data_bit,
    }
  } as u64;
  unsafe { core::arch::asm!("msr csselr_el1, {0:x}", in(reg) csselr_el1) };
  asm::barrier::instruction_synchronization!();

  // https://developer.arm.com/documentation/100442/0100/register-descriptions/aarch64-system-registers/ccsidr-el1--cache-size-id-register--el1?lang=en
  let ccsidr_el1: u32;
  unsafe { core::arch::asm!("mrs {0:x}, ccsidr_el1", out(reg) ccsidr_el1) };

  let line_size_bytes = 1u32 << (bit_of_range::<2, 0>(ccsidr_el1) + 4);
  let associativity = bit_of_range::<12, 3>(ccsidr_el1) + 1;
  let num_sets = bit_of_range::<27, 13>(ccsidr_el1) + 1;
  CacheInfo {
    exists: ccsidr_el1 != 0,
    level: cache_level,
    cache_type,
    line_size_bytes,
    associativity,
    num_sets,
    total_size_bytes: line_size_bytes * associativity * num_sets,
    write_alloc_supported: bit_of::<28>(ccsidr_el1) != 0,
    read_alloc_supported: bit_of::<29>(ccsidr_el1) != 0,
    write_back_supported: bit_of::<30>(ccsidr_el1) != 0,
    write_through_supported: bit_of::<31>(ccsidr_el1) != 0,
  }
}

fn read_sctrl_reg() -> SCTRL {
  let sctrl_el1: u32;
  unsafe { core::arch::asm!("mrs {0:x}, sctlr_el1", out(reg) sctrl_el1) };
  SCTRL {
    m: bit_of::<0>(sctrl_el1) != 0,
  }
}

pub fn get_memory_model() -> MemoryModel {
  MemoryModel {
    cache: arrayvec::ArrayVec::from_iter([
      read_cache_info(CacheLevel::L1, CacheType::Data),
      read_cache_info(CacheLevel::L1, CacheType::Instruction),
      read_cache_info(CacheLevel::L2, CacheType::Unified),
      read_cache_info(CacheLevel::L3, CacheType::Unified),
    ]),
    mmu_enabled: read_sctrl_reg().m,
  }
}

pub fn get_ring_level() -> u32 {
  let mut level: u32;
  unsafe { core::arch::asm!("mrs {0:x}, CurrentEL", out(reg) level) };
  return bit::bit_of_range::<3, 2>(level);
}
