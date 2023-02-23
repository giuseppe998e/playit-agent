use std::io::{Result, Write};

use byteorder::{BigEndian, WriteBytesExt};

use crate::socket::{Port, Protocol, Socket};

use super::MessageEncode;

impl MessageEncode for Socket {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        self.ip.write_into(buf)?;
        self.port.write_into(buf)?;
        self.proto.write_into(buf)
    }
}

impl MessageEncode for Port {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        let (start, end) = match self {
            Self::Single(port) => (port, port),
            Self::Range(range) => (*range.start(), *range.end()),
        };

        buf.write_u16::<BigEndian>(start)?;
        buf.write_u16::<BigEndian>(end)
    }
}

impl MessageEncode for Protocol {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u8(self.into())
    }
}
