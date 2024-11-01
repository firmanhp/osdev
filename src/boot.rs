#[cfg(feature = "device")]
core::arch::global_asm!(include_str!("boot.S"));
