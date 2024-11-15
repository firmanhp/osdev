static mut OPS: core::mem::MaybeUninit<Ops> =
  core::mem::MaybeUninit::<Ops>::uninit();
// This will be set to 0 during bss zero-ing.
static mut SET: bool = false;

pub struct Ops {
  pub get_memory_model: fn() -> MemoryModel,
  pub get_ring_level: fn() -> u32,
}

#[derive(PartialEq, Eq)]
pub enum CacheType {
  Data,
  Instruction,
  Unified,
}

#[derive(PartialEq, Eq)]
pub enum CacheLevel {
  L1,
  L2,
  L3,
}

impl core::fmt::Display for CacheType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      CacheType::Data => write!(f, "data"),
      CacheType::Instruction => write!(f, "instruction"),
      CacheType::Unified => write!(f, "unified"),
    }
  }
}

impl core::fmt::Display for CacheLevel {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      CacheLevel::L1 => write!(f, "L1"),
      CacheLevel::L2 => write!(f, "L2"),
      CacheLevel::L3 => write!(f, "L3"),
    }
  }
}

pub struct CacheInfo {
  pub exists: bool,
  pub level: CacheLevel,
  pub cache_type: CacheType,
  pub line_size_bytes: u32,
  pub associativity: u32,
  pub num_sets: u32,
  pub total_size_bytes: u32,
  pub write_alloc_supported: bool,
  pub read_alloc_supported: bool,
  pub write_back_supported: bool,
  pub write_through_supported: bool,
}

pub struct MemoryModel {
  pub cache: arrayvec::ArrayVec<CacheInfo, 8>,
  pub mmu_enabled: bool,
}

#[inline(always)]
pub fn get_memory_model() -> MemoryModel {
  unsafe {
    assert!(SET, "No impl");
    (OPS.assume_init_ref().get_memory_model)()
  }
}

#[inline(always)]
pub fn get_ring_level() -> u32 {
  unsafe {
    assert!(SET, "No impl");
    (OPS.assume_init_ref().get_ring_level)()
  }
}

pub fn set_impl(ops: Ops) {
  unsafe {
    OPS = core::mem::MaybeUninit::<Ops>::new(ops);
    SET = true;
  };
}
