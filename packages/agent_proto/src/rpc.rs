use std::io::{Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::MessageEncoding;

#[derive(Debug, Eq, PartialEq)]
pub struct ControlRpcMessage<T: MessageEncoding> {
    pub request_id: u64,
    pub content: T,
}

impl<T: MessageEncoding> MessageEncoding for ControlRpcMessage<T> {
    fn write_to<I: Write>(&self, out: &mut I) -> std::io::Result<()> {
        out.write_u64::<BigEndian>(self.request_id)?;
        self.content.write_to(out)
    }

    fn read_from<I: Read>(read: &mut I) -> std::io::Result<Self> {
        Ok(ControlRpcMessage {
            request_id: read.read_u64::<BigEndian>()?,
            content: T::read_from(read)?,
        })
    }
}
