use core::mem;
use std::{
    io::{self, Error, ErrorKind},
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};

use bytes::{Buf, BufMut};

use crate::codec::{Decode, Encode};

pub const FLOW_V6_ID: u64 = 0x6668_676F_6861_6366;
pub const FLOW_V4_ID: u64 = 0x4448_474F_4841_4344;
pub const FLOW_V4_ID_OLD: u64 = 0x5CB8_67CF_7881_73B2;

const FLOW_ID_SIZE: usize = mem::size_of::<u64>();
const FLOW_V4_SIZE: usize = 12;
const FLOW_V6_SIZE: usize = 40;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketFlow {
    V4(SocketFlowV4),
    V6(SocketFlowV6),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SocketFlowV4 {
    src: SocketAddrV4,
    dest: SocketAddrV4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SocketFlowV6 {
    src: SocketAddrV6,
    dest: SocketAddrV6,
    // XXX Does it refer to src or dest?
    flowinfo: u32,
}

impl SocketFlow {
    pub fn src(&self) -> SocketAddr {
        match self {
            Self::V4(flow) => flow.src.into(),
            Self::V6(flow) => flow.src.into(),
        }
    }

    pub fn dest(&self) -> SocketAddr {
        match self {
            Self::V4(flow) => flow.dest.into(),
            Self::V6(flow) => flow.dest.into(),
        }
    }

    pub fn flowinfo(&self) -> Option<u32> {
        match self {
            SocketFlow::V6(flow) => Some(flow.flowinfo),
            SocketFlow::V4(_) => None,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::V4(flow) => Self::V4(flow.flip()),
            Self::V6(flow) => Self::V6(flow.flip()),
        }
    }

    pub fn size(&self) -> usize {
        let inner = match self {
            SocketFlow::V4(_) => SocketFlowV4::size(),
            SocketFlow::V6(_) => SocketFlowV6::size(),
        };

        inner + FLOW_ID_SIZE
    }

    pub fn is_ipv4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::V6(_))
    }
}

impl Encode for SocketFlow {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > FLOW_ID_SIZE);

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

impl Decode for SocketFlow {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        let buf_chunk = buf.chunk();
        let buf_remaining = buf.remaining();

        // V4
        {
            crate::codec::ensure!(buf_remaining >= SocketFlowV4::size() + FLOW_ID_SIZE);

            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV4::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if matches!(footer_id, FLOW_V4_ID | FLOW_V4_ID_OLD) {
                buf.advance(SocketFlowV4::size() + FLOW_ID_SIZE);
                return Ok(());
            }
        }

        // V6
        {
            crate::codec::ensure!(buf_remaining >= SocketFlowV6::size() + FLOW_ID_SIZE);

            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV6::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if footer_id == FLOW_V6_ID {
                buf.advance(SocketFlowV6::size() + FLOW_ID_SIZE);
                return Ok(());
            }
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "unknown discriminant for 'SocketFlow'",
        ))
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let buf_chunk = buf.chunk();

        // V4
        {
            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV4::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if matches!(footer_id, FLOW_V4_ID | FLOW_V4_ID_OLD) {
                let flow = SocketFlowV4::decode(buf);
                buf.advance(FLOW_ID_SIZE);
                return Self::V4(flow);
            }
        }

        // V6
        {
            let footer_id = {
                let bytes = &buf_chunk[SocketFlowV6::size()..];
                let bytes = unsafe { *(bytes as *const _ as *const [_; FLOW_ID_SIZE]) };
                u64::from_be_bytes(bytes)
            };

            if footer_id == FLOW_V6_ID {
                let flow = SocketFlowV6::decode(buf);
                buf.advance(FLOW_ID_SIZE);
                return Self::V6(flow);
            }
        }

        panic!("unknown discriminant for 'SocketFlow'")
    }
}

impl SocketFlowV4 {
    pub fn new(src: SocketAddrV4, dest: SocketAddrV4) -> Self {
        Self { src, dest }
    }

    pub fn src(&self) -> &SocketAddrV4 {
        &self.src
    }

    pub fn dest(&self) -> &SocketAddrV4 {
        &self.dest
    }

    pub fn flip(self) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
        }
    }

    pub const fn size() -> usize {
        FLOW_V4_SIZE
    }
}

impl From<SocketFlowV4> for SocketFlow {
    fn from(value: SocketFlowV4) -> Self {
        Self::V4(value)
    }
}

impl Encode for SocketFlowV4 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(
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

impl Decode for SocketFlowV4 {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::checked_advance!(
            buf.remaining() >= mem::size_of::<u32>() * 2 + mem::size_of::<u16>() * 2
        );

        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let src_ip = <u32>::decode(buf);
        let dest_ip = <u32>::decode(buf);
        let src_port = <u16>::decode(buf);
        let dest_port = <u16>::decode(buf);

        let src = SocketAddrV4::new(src_ip.into(), src_port);
        let dest = SocketAddrV4::new(dest_ip.into(), dest_port);

        Self::new(src, dest)
    }
}

impl SocketFlowV6 {
    pub fn new(src: SocketAddrV6, dest: SocketAddrV6, flowinfo: u32) -> Self {
        Self {
            src,
            dest,
            flowinfo,
        }
    }

    pub fn src(&self) -> &SocketAddrV6 {
        &self.src
    }

    pub fn dest(&self) -> &SocketAddrV6 {
        &self.dest
    }

    pub fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    pub fn flip(self) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            flowinfo: self.flowinfo,
        }
    }

    pub const fn size() -> usize {
        FLOW_V6_SIZE
    }
}

impl From<SocketFlowV6> for SocketFlow {
    fn from(value: SocketFlowV6) -> Self {
        Self::V6(value)
    }
}

impl Encode for SocketFlowV6 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(
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

impl Decode for SocketFlowV6 {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::checked_advance!(
            buf.remaining()
                >= mem::size_of::<u128>() * 2 + mem::size_of::<u16>() * 2 + mem::size_of::<u32>()
        );

        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let src_ip = <u128>::decode(buf);
        let dest_ip = <u128>::decode(buf);
        let src_port = <u16>::decode(buf);
        let dest_port = <u16>::decode(buf);
        let flowinfo = <u32>::decode(buf);

        let src = SocketAddrV6::new(src_ip.into(), src_port, 0, 0);
        let dest = SocketAddrV6::new(dest_ip.into(), dest_port, 0, 0);

        Self::new(src, dest, flowinfo)
    }
}
