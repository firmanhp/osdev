pub(self) mod bcm2837_raspberrypi_3b {
  mod board_setup;
  mod panic;
}

pub(self) mod bcm_raspberrypi_common {
  pub(super) mod board_info;
  pub(super) mod board_type;
  pub(super) mod mmio;
  pub(super) mod network;
}
