use arrayvec::ArrayVec;

use super::board_type;
use crate::container::arrayvec_extensions;
use crate::io::mailbox;
use crate::metadata::board;
use crate::metadata::network;

fn get_board_info() -> board::BoardInfo {
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

  let board_type = board_type::raspi_board_type();
  let board_serial = mailbox::tag::HwGetBoardSerial::read_response(&message)
    .unwrap()
    .serial();

  board::BoardInfo {
    model: arrayvec_extensions::capped_format!(32, "Raspberry_{}", board_type),
    serial: arrayvec_extensions::capped_format!(32, "{:010x}", board_serial),
    attributes: pull_board_attributes(&message),
  }
}

#[inline(always)]
fn pull_board_attributes<const N: usize>(
  message: &mailbox::Message<N>,
) -> ArrayVec<board::BoardAttribute, { board::ATTRIBUTES_CAP }> {
  let arm_mem_base_address: u32;
  let arm_mem_size_bytes: u32;
  let videocore_mem_base_address: u32;
  let videocore_mem_size_bytes: u32;
  let vendor_model = mailbox::tag::HwGetBoardModel::read_response(message)
    .unwrap()
    .board_model();
  let mac_address = network::get_mac_address();
  let videocore_firmware_rev =
    mailbox::tag::VideocoreGetFirmwareRevision::read_response(message)
      .unwrap()
      .firmware_rev();

  {
    let response =
      mailbox::tag::HwGetArmMemory::read_response(message).unwrap();
    arm_mem_base_address = response.base_address();
    arm_mem_size_bytes = response.size_bytes();
  }
  {
    let response =
      mailbox::tag::HwGetVideocoreMemory::read_response(message).unwrap();
    videocore_mem_base_address = response.base_address();
    videocore_mem_size_bytes = response.size_bytes();
  }

  ArrayVec::from_iter([
    board::BoardAttribute::of(
      "vendor_model",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{:08x}",
        vendor_model
      ),
    ),
    board::BoardAttribute::of(
      "vc_firmware_rev",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{}",
        videocore_firmware_rev
      ),
    ),
    board::BoardAttribute::of(
      "arm_mem_base",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{:#010X}",
        arm_mem_base_address
      ),
    ),
    board::BoardAttribute::of(
      "arm_mem_size_kib",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{}",
        arm_mem_size_bytes / 1024
      ),
    ),
    board::BoardAttribute::of(
      "vc_mem_base",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{:#010X}",
        videocore_mem_base_address
      ),
    ),
    board::BoardAttribute::of(
      "vc_mem_size_kib",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{}",
        videocore_mem_size_bytes / 1024
      ),
    ),
    board::BoardAttribute::of(
      "mac_address",
      arrayvec_extensions::capped_format!(
        { board::ATTRIBUTE_VAL_CAP },
        "{}",
        mac_address
      ),
    ),
  ])
}

pub fn initialize() {
  board::set_impl(board::Ops { get_board_info });
}
