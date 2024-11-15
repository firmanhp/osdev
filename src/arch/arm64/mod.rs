pub mod asm;
mod kernel {
  mod common_setup;
  mod head;
  mod interrupt;
  pub mod interrupt_handle;
}
pub(self) mod metadata {
  pub(super) mod cpu;
}
mod vendor {
  mod broadcom;
}
