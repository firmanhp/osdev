use super::interrupt_handle;
use crate::metadata;

#[cfg(feature = "device")]
#[no_mangle]
extern "C" fn arch_setup() {
  metadata::cpu::set_impl(metadata::cpu::Ops {
    get_memory_model: crate::arch::arm64::metadata::cpu::get_memory_model,
  });
  // interrupt_handle::initialize();
  // interrupt_handle::enable_irq();
}
