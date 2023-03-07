use core::mem;
use std::{
    io::{self, Error, ErrorKind},
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
};

use bytes::Buf;

use super::Decode;
use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_ID_SIZE, FLOW_V4_ID,
    FLOW_V4_ID_OLD, FLOW_V6_ID,
};

// Socket
impl Decode for Socket {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let ip = IpAddr::decode(buf)?;
        let port = Port::decode(buf)?;
        let proto = Protocol::decode(buf)?;

        Ok(Self { ip, port, proto })
    }
}

// Port
impl Decode for Port {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() >= mem::size_of::<u16>() * 2);
        let start = buf.get_u16();
        let end = buf.get_u16();

        match start == end {
            true => Ok(Self::Single(start)),
            false => Ok(Self::Range(start..=end)),
        }
    }
}

// Protocol
impl Decode for Protocol {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u8>::decode(buf)?;
        match discriminant {
            1 => Ok(Self::Tcp),
            2 => Ok(Self::Udp),
            3 => Ok(Self::Both),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'Protocol'",
            )),
        }
    }
}

// SocketFlow
impl Decode for SocketFlow {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let buf_chunk = buf.chunk();
        let buf_remaining = buf.remaining();

        // V4
        {
            super::ensure!(buf_remaining >= SocketFlowV4::size() + FLOW_ID_SIZE);

            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV4::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if matches!(footer_id, FLOW_V4_ID | FLOW_V4_ID_OLD) {
                return SocketFlowV4::decode(buf).map(Self::V4);
            }
        }

        // V6
        {
            super::ensure!(buf_remaining >= SocketFlowV6::size() + FLOW_ID_SIZE);

            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV6::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if footer_id == FLOW_V6_ID {
                return SocketFlowV6::decode(buf).map(Self::V6);
            }
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "unknown discriminant for 'SocketFlow'",
        ))
    }
}

impl Decode for SocketFlowV4 {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::super::ensure!(
            buf.remaining() >= mem::size_of::<u32>() * 2 + mem::size_of::<u16>() * 2
        );

        let src_ip = buf.get_u32();
        let dest_ip = buf.get_u32();
        let src_port = buf.get_u16();
        let dest_port = buf.get_u16();

        let src = SocketAddrV4::new(src_ip.into(), src_port);
        let dest = SocketAddrV4::new(dest_ip.into(), dest_port);

        Ok(Self::new(src, dest))
    }
}

impl Decode for SocketFlowV6 {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(
            buf.remaining()
                >= mem::size_of::<u128>() * 2 + mem::size_of::<u16>() * 2 + mem::size_of::<u32>()
        );

        let src_ip = buf.get_u128();
        let dest_ip = buf.get_u128();
        let src_port = buf.get_u16();
        let dest_port = buf.get_u16();
        let flowinfo = buf.get_u32();

        let src = SocketAddrV6::new(src_ip.into(), src_port, flowinfo, 0);
        let dest = SocketAddrV6::new(dest_ip.into(), dest_port, flowinfo, 0);

        Ok(Self::new(src, dest, flowinfo))
    }
}
