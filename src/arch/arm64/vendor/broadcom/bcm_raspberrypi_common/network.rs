use crate::io::mailbox;
use crate::metadata::network::{self, MacAddress};

fn get_mac_address() -> MacAddress {
  let hw_board_mac_address_tag =
    mailbox::tag::HwGetBoardMacAddress::Request {}.to_tag();
  let message =
    mailbox::send(
      mailbox::Message::<
        { mailbox::tag::HwGetBoardMacAddress::Tag::MESSAGE_LEN },
      >::builder()
      .add_tag(&hw_board_mac_address_tag)
      .build(),
    );

  MacAddress {
    data: mailbox::tag::HwGetBoardMacAddress::read_response(&message)
      .unwrap()
      .mac_address(),
  }
}

pub fn initialize() {
  network::set_impl(network::Ops { get_mac_address });
}
