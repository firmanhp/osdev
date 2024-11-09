pub mod asm;
mod kernel {
  mod common_setup;
  mod head;
}
pub(self) mod metadata {
  pub(super) mod cpu;
}
mod vendor {
  mod broadcom;
}
