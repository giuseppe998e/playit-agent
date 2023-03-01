use std::io::{Result, Write};

use byteorder::{BigEndian, WriteBytesExt};

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_V4_ID_OLD, FLOW_V6_ID,
};

use super::MessageEncode;

impl MessageEncode for Socket {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        self.ip.write_into(buf)?;
        self.port.write_into(buf)?;
        self.proto.write_into(buf)
    }
}

impl MessageEncode for Port {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        let (start, end) = match self {
            Self::Single(port) => (port, port),
            Self::Range(range) => (*range.start(), *range.end()),
        };

        buf.write_u16::<BigEndian>(start)?;
        buf.write_u16::<BigEndian>(end)
    }
}

impl MessageEncode for Protocol {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_u8(self.into())
    }
}

// SocketFlow
impl MessageEncode for SocketFlow {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        match self {
            SocketFlow::V4(flow) => {
                flow.write_into(buf)?;
                buf.write_u64::<BigEndian>(FLOW_V4_ID_OLD)
            }
            SocketFlow::V6(flow) => {
                flow.write_into(buf)?;
                buf.write_u64::<BigEndian>(FLOW_V6_ID)
            }
        }
    }
}

impl MessageEncode for SocketFlowV4 {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        let src = self.src();
        let dest = self.dest();

        buf.write_u32::<BigEndian>((*src.ip()).into())?;
        buf.write_u32::<BigEndian>((*dest.ip()).into())?;
        buf.write_u16::<BigEndian>(src.port())?;
        buf.write_u16::<BigEndian>(dest.port())
    }
}

impl MessageEncode for SocketFlowV6 {
    fn write_into<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        let src = self.src();
        let dest = self.dest();
        let flowinfo = self.flowinfo();

        buf.write_u128::<BigEndian>((*src.ip()).into())?;
        buf.write_u128::<BigEndian>((*dest.ip()).into())?;
        buf.write_u16::<BigEndian>(src.port())?;
        buf.write_u16::<BigEndian>(dest.port())?;
        buf.write_u32::<BigEndian>(flowinfo)
    }
}
