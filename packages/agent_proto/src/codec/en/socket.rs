use core::mem;
use std::io;

use bytes::BufMut;

use super::Encode;
use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_ID_SIZE, FLOW_V4_ID_OLD,
    FLOW_V6_ID,
};

// Socket
impl Encode for Socket {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.ip.encode(buf)?;
        self.port.encode(buf)?;
        self.proto.encode(buf)
    }
}

// Port
impl Encode for Port {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() >= mem::size_of::<u16>() * 2);

        let (start, end) = match self {
            Self::Single(port) => (port, port),
            Self::Range(range) => (*range.start(), *range.end()),
        };

        buf.put_u16(start);
        buf.put_u16(end);

        Ok(())
    }
}

// Protocol
impl Encode for Protocol {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        let byte: u8 = self.into();
        byte.encode(buf)
    }
}

// SocketFlow
impl Encode for SocketFlow {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > FLOW_ID_SIZE);

        match self {
            SocketFlow::V4(flow) => {
                flow.encode(buf)?;
                buf.put_u64(FLOW_V4_ID_OLD);
            }
            SocketFlow::V6(flow) => {
                flow.encode(buf)?;
                buf.put_u64(FLOW_V6_ID);
            }
        }

        Ok(())
    }
}

impl Encode for SocketFlowV4 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(
            buf.remaining_mut() >= mem::size_of::<u32>() * 2 + mem::size_of::<u16>() * 2
        );

        let (src_ip, src_port) = {
            let src = self.src();
            (*src.ip(), src.port())
        };

        let (dest_ip, dest_port) = {
            let dest = self.dest();
            (*dest.ip(), dest.port())
        };

        buf.put_u32(src_ip.into());
        buf.put_u32(dest_ip.into());
        buf.put_u16(src_port);
        buf.put_u16(dest_port);

        Ok(())
    }
}

impl Encode for SocketFlowV6 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(
            buf.remaining_mut()
                >= mem::size_of::<u128>() * 2 + mem::size_of::<u16>() * 2 + mem::size_of::<u32>()
        );

        let (src_ip, src_port) = {
            let src = self.src();
            (*src.ip(), src.port())
        };

        let (dest_ip, dest_port) = {
            let dest = self.dest();
            (*dest.ip(), dest.port())
        };

        let flowinfo = self.flowinfo();

        buf.put_u128(src_ip.into());
        buf.put_u128(dest_ip.into());
        buf.put_u16(src_port);
        buf.put_u16(dest_port);
        buf.put_u32(flowinfo);

        Ok(())
    }
}
