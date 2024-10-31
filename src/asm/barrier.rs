#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
  // https://developer.arm.com/documentation/100941/0101/Barriers

  // Instruction Synchronization Barrier(ISB) is used to guarantee that any
  // subsequent instructions are fetched, so that privilege and access are
  // checked with the current MMU configuration. It is used to ensure any
  // previously executed context-changing operations, such as writes to system
  // control registers, have completed by the time the ISB completes.
  #[macro_export]
  macro_rules! instruction_synchronization {
    () => {
      unsafe {
        core::arch::asm!("isb", options(nostack, preserves_flags));
      }
    };
  }

  // Data Memory Barrier (DMB) prevents reordering of data accesses instructions
  // across the DMB instruction. Depending on the barrier type, certain data
  // accesses, that is, loads or stores, but not instruction fetches, performed
  // by this processor before the DMB, are visible to all other masters within
  // the specified shareability domain before certain other data accesses after
  // the DMB.
  #[macro_export]
  macro_rules! data_memory {
    ($param:expr) => {
      unsafe {
        core::arch::asm!(
          core::concat!("dmb ", $param),
          options(nostack, preserves_flags)
        );
      }
    };
  }

  // Data Synchronization Barrier(DSB) enforces the same ordering as the Data
  // Memory Barrier, but it also blocks execution of any further instructions,
  // not just loads or stores, until synchronization is complete. This can be
  // used to prevent execution of a SEV instruction, for instance, that would
  // signal to other cores that an event occurred. It waits until all cache,
  // TLB, and branch predictor maintenance operations that are issued by this
  // processor have completed for the specified shareability domain.
  #[macro_export]
  macro_rules! data_synchronization {
    ($param:expr) => {
      unsafe {
        core::arch::asm!(
          core::concat!("dsb ", $param),
          options(nostack, preserves_flags)
        );
      }
    };
  }

  
  
  pub use instruction_synchronization;
}
