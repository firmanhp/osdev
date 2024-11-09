use crate::io::mailbox;
use crate::metadata::network;

fn get_mac_address() -> network::MacAddress {
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

  network::MacAddress {
    data: mailbox::tag::HwGetBoardMacAddress::read_response(&message)
      .unwrap()
      .mac_address(),
  }
}

pub fn initialize() {
  network::set_impl(network::Ops { get_mac_address });
}
