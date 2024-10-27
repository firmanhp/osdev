#[macro_export]
macro_rules! make_tag {
  ($tag_name:ident, $tag_id:expr,
   request { $( $req_ident:ident : $req_type:ty ),* },
   response { $( $resp_ident:ident : $resp_type:ty ),* }) => {

    // concat_ident is a nightly feature, so wrap it inside mod first
    #[allow(non_snake_case)]
    pub mod $tag_name {
      use crate::io::mailbox::tag::TagId;
      use crate::io::mailbox::MessageView;
      use crate::common::error::ErrorKind;

      // These request/response structs are packed, meaning that Rust does not
      // put any alignment. Accessing the data requires
      // read_unaligned(addr_of!(...)).
      #[repr(C, packed)]
      #[derive(Copy, Clone)]
      pub struct Request {
        $(pub $req_ident: $req_type,)*
      }

      #[repr(C, packed)]
      #[derive(Copy, Clone)]
      pub struct Response {
        $(pub $resp_ident: $resp_type,)*
      }

      // Align to 32bits
      #[repr(C, align(4))]
      pub union Payload {
        pub request: Request,
        pub response: Response,
      }

      /*
      Tag format:
        u32: tag identifier
        u32: value buffer size in bytes
        u32:
      Request codes:
        b31 clear: request
        b30-b0: reserved
      Response codes:
        b31 set: response
        b30-b0: value length in bytes

      u8...: value buffer
      u8...: padding to align the tag to 32 bits.
      */
      #[repr(C, align(4))]
      pub struct Tag {
        // Tag identifier
        pub id: u32,
        // Value buffer size in bytes
        pub payload_size_bytes: u32,
        // [31] --> Response: 1, Request: 0
        // [30:0] --> Response: Value len (bytes)
        pub code: u32,
        pub payload: Payload,
      }

      impl Into<Tag> for Request {
        fn into(self) -> Tag { self.to_tag() }
      }

      impl Request {
        pub fn to_tag(self) -> Tag { make(self) }
      }

      impl Response {
        // Helper function to pull members
        $(pub fn $resp_ident(&self) -> $resp_type {
          unsafe {
            core::ptr::read_unaligned(core::ptr::addr_of!(self.$resp_ident))
          }
        })*
      }

      impl Tag {
        pub const SIZE_BYTES: usize = core::mem::size_of::<Self>();
        pub const MESSAGE_LEN: usize = (Self::SIZE_BYTES / 4);
        pub const PAYLOAD_SIZE_BYTES: usize = core::mem::size_of::<Payload>();
      }

      fn make(request: Request) -> Tag {
        // Assertion
        let tag_id: TagId = $tag_id;
        Tag {
          id: tag_id as u32,
          payload_size_bytes: Tag::PAYLOAD_SIZE_BYTES as u32,
          code: 0x0000_0000,
          payload: Payload { request }
        }
      }

      pub fn read_response(message: &dyn MessageView) -> Result<&Response, ErrorKind> {
        let tag_buf = message.tag_buffer_lookup($tag_id as u32)?;
        if tag_buf.len() != core::mem::size_of::<Tag>() {
          panic!("Unexpected mailbox tag size");
        }
        // This should pass. Tag payloads are aligned to 4bytes, including the metadata.
        // The message containing the tags are 16 bytes aligned.
        if tag_buf.as_ptr() as usize % core::mem::align_of::<Tag>() != 0 {
          panic!("Tag buffer has wrong alignment");
        }

        let tag = unsafe { &*(tag_buf.as_ptr() as *const Tag) };
        if tag.id != $tag_id as u32 {
          panic!("Received tag with wrong ID");
        }
        if tag.payload_size_bytes != Tag::PAYLOAD_SIZE_BYTES as u32 {
          panic!("Payload size is invalid");
        }
        if (tag.code >> 31 != 1) {
          // This is not a response.
          return Err(ErrorKind::InvalidInput);
        }
        if (tag.code & !(1 << 31)) as usize != core::mem::size_of::<Response>() {
          panic!("Response length is invalid");
        }
        unsafe { Ok(&tag.payload.response) }
      }

      impl crate::io::mailbox::tag::MessageTag for Tag {
        fn size_bytes(&self) -> usize { Self::SIZE_BYTES }
        fn payload_size_bytes(&self) -> usize { Self::PAYLOAD_SIZE_BYTES }
      }
    }
  };
}

pub use make_tag;
