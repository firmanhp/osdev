// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
// https://github.com/raspberrypi/firmware/wiki/Mailboxes
// https://github.com/raspberrypi/firmware/wiki/Accessing-mailboxes

/*
Mailboxes facilitate communication between the ARM and the VideoCore.
Each mailbox is an 8-deep FIFO of 32-bit words, which can be read (popped) or
written (pushed) by the ARM and VC.
Only mailbox 0's status can trigger interrupts on the ARM,
so MB 0 is always for communication from VC to ARM
and MB 1 is for ARM to VC.

The ARM should never write MB 0 or read MB 1.
*/

#[macro_use]
mod macros;
mod message;
pub mod tag;

pub use message::send;
pub use message::Message;
pub use message::MessageView;
