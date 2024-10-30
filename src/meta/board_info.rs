use crate::io::mailbox;

#[repr(u32)]
pub enum RaspiBoardType {
  Unknown = 0,
  Pi1 = 0x0B76,
  Pi2 = 0x0C07,
  Pi3 = 0x0D03,
  Pi4 = 0x0D08,
}

impl From<u32> for RaspiBoardType {
  fn from(val: u32) -> RaspiBoardType {
    match val {
      x if x == RaspiBoardType::Pi1 as u32 => RaspiBoardType::Pi1,
      x if x == RaspiBoardType::Pi2 as u32 => RaspiBoardType::Pi2,
      x if x == RaspiBoardType::Pi3 as u32 => RaspiBoardType::Pi3,
      x if x == RaspiBoardType::Pi4 as u32 => RaspiBoardType::Pi4,
      _ => RaspiBoardType::Unknown,
    }
  }
}

impl core::fmt::Display for RaspiBoardType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      RaspiBoardType::Unknown => write!(f, "UNKNOWN"),
      RaspiBoardType::Pi1 => write!(f, "Pi1"),
      RaspiBoardType::Pi2 => write!(f, "Pi2"),
      RaspiBoardType::Pi3 => write!(f, "Pi3"),
      RaspiBoardType::Pi4 => write!(f, "Pi4"),
    }
  }
}

pub struct MacAddress {
  data: [u8; 6],
}

impl core::fmt::Display for MacAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
      self.data[0],
      self.data[1],
      self.data[2],
      self.data[3],
      self.data[4],
      self.data[5]
    )
  }
}

pub struct RaspiBoardInfo {
  pub videocore_firmware_rev: u32,
  pub board_type: RaspiBoardType,
  pub board_model: u32,
  // In network byte order
  pub board_mac_address: MacAddress,
  pub board_serial: u64,
  pub arm_mem_base_address: u32,
  pub arm_mem_size_bytes: u32,
  pub videocore_mem_base_address: u32,
  pub videocore_mem_size_bytes: u32,
}

// https://wiki.osdev.org/Detecting_Raspberry_Pi_Board
#[cfg(target_arch = "aarch64")]
pub fn raspi_board_type() -> RaspiBoardType {
  let mut id: u32;
  // Move System Register
  // https://developer.arm.com/documentation/ddi0602/2024-06/Base-Instructions/MRS--Move-System-register-to-general-purpose-register-?lang=en
  // MIDR_EL1
  // https://developer.arm.com/documentation/ddi0601/2024-06/External-Registers/MIDR-EL1--Main-ID-Register?lang=en
  unsafe { core::arch::asm!("mrs {0:x}, midr_el1", out(reg) id) };
  let part_num: u32 = (id >> 4) & 0xFFF;
  return RaspiBoardType::from(part_num);
}

pub fn raspi_board_info() -> RaspiBoardInfo {
  let vc_fw_rev_tag =
    mailbox::tag::VideocoreGetFirmwareRevision::Request {}.to_tag();
  let hw_board_model_tag = mailbox::tag::HwGetBoardModel::Request {}.to_tag();
  let hw_board_rev_tag = mailbox::tag::HwGetBoardRevision::Request {}.to_tag();
  let hw_board_mac_address_tag =
    mailbox::tag::HwGetBoardMacAddress::Request {}.to_tag();
  let hw_board_serial_tag = mailbox::tag::HwGetBoardSerial::Request {}.to_tag();
  let hw_arm_memory_tag = mailbox::tag::HwGetArmMemory::Request {}.to_tag();
  let hw_vc_memory_tag =
    mailbox::tag::HwGetVideocoreMemory::Request {}.to_tag();

  let message = mailbox::send(
    mailbox::Message::<
      {
        mailbox::tag::VideocoreGetFirmwareRevision::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetBoardModel::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetBoardRevision::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetBoardMacAddress::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetBoardSerial::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetArmMemory::Tag::MESSAGE_LEN
          + mailbox::tag::HwGetVideocoreMemory::Tag::MESSAGE_LEN
      },
    >::builder()
    .add_tag(&vc_fw_rev_tag)
    .add_tag(&hw_board_model_tag)
    .add_tag(&hw_board_rev_tag)
    .add_tag(&hw_board_mac_address_tag)
    .add_tag(&hw_board_serial_tag)
    .add_tag(&hw_arm_memory_tag)
    .add_tag(&hw_vc_memory_tag)
    .build(),
  );

  let arm_mem_base_address: u32;
  let arm_mem_size_bytes: u32;
  let videocore_mem_base_address: u32;
  let videocore_mem_size_bytes: u32;

  {
    let response =
      mailbox::tag::HwGetArmMemory::read_response(&message).unwrap();
    arm_mem_base_address = response.base_address();
    arm_mem_size_bytes = response.size_bytes();
  }
  {
    let response =
      mailbox::tag::HwGetVideocoreMemory::read_response(&message).unwrap();
    videocore_mem_base_address = response.base_address();
    videocore_mem_size_bytes = response.size_bytes();
  }
  RaspiBoardInfo {
    videocore_firmware_rev:
      mailbox::tag::VideocoreGetFirmwareRevision::read_response(&message)
        .unwrap()
        .firmware_rev(),
    board_type: raspi_board_type(),
    board_model: mailbox::tag::HwGetBoardModel::read_response(&message)
      .unwrap()
      .board_model(),
    // In network byte order
    board_mac_address: MacAddress {
      data: mailbox::tag::HwGetBoardMacAddress::read_response(&message)
        .unwrap()
        .mac_address(),
    },
    board_serial: mailbox::tag::HwGetBoardSerial::read_response(&message)
      .unwrap()
      .serial(),
    arm_mem_base_address,
    arm_mem_size_bytes,
    videocore_mem_base_address,
    videocore_mem_size_bytes,
  }
}
