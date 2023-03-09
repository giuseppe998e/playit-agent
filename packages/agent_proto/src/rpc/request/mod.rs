mod ping;
mod port_map;
mod register;

use core::mem;
use std::io::{self, Error, ErrorKind};

use bytes::{Buf, BufMut};

use crate::{
    codec::{Decode, Encode},
    rpc::common::AgentSession,
};

pub use self::{ping::Ping, port_map::PortMappingRequest, register::RegisterRequest};

pub type KeepAliveRequest = AgentSession;
pub type UdpChannelRequest = AgentSession;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcRequest {
    Ping(Ping),

    KeepAlive(KeepAliveRequest),

    Register(RegisterRequest),
    UdpChannel(UdpChannelRequest),
    PortMapping(PortMappingRequest),
}

impl RpcRequest {
    pub const PING_IDX: u8 = 1;

    pub const KEEP_ALIVE_IDX: u8 = 3;

    pub const REGISTER_IDX: u8 = 2;
    pub const UPD_CHANNEL_IDX: u8 = 4;
    pub const PORT_MAPPING_IDX: u8 = 5;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::Ping(_) => Self::PING_IDX,

            Self::KeepAlive(_) => Self::KEEP_ALIVE_IDX,

            Self::Register(_) => Self::REGISTER_IDX,
            Self::UdpChannel(_) => Self::UPD_CHANNEL_IDX,
            Self::PortMapping(_) => Self::PORT_MAPPING_IDX,
        }
    }
}

impl Encode for RpcRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

        match self {
            Self::Ping(req) => {
                buf.put_u32(Self::PING_IDX as u32);
                req.encode(buf)
            }

            Self::KeepAlive(req) => {
                buf.put_u32(Self::KEEP_ALIVE_IDX as u32);
                req.encode(buf)
            }

            Self::Register(req) => {
                buf.put_u32(Self::REGISTER_IDX as u32);
                req.encode(buf)
            }
            Self::UdpChannel(req) => {
                buf.put_u32(Self::UPD_CHANNEL_IDX as u32);
                req.encode(buf)
            }
            Self::PortMapping(req) => {
                buf.put_u32(Self::PORT_MAPPING_IDX as u32);
                req.encode(buf)
            }
        }
    }
}

impl Decode for RpcRequest {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining() > mem::size_of::<u32>());
        let discriminant = <u32>::decode(buf) as u8;

        match discriminant {
            Self::PING_IDX => Ping::check(buf),

            Self::KEEP_ALIVE_IDX => KeepAliveRequest::check(buf),

            Self::REGISTER_IDX => RegisterRequest::check(buf),
            Self::UPD_CHANNEL_IDX => UdpChannelRequest::check(buf),
            Self::PORT_MAPPING_IDX => PortMappingRequest::check(buf),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'RpcRequest'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u32>::decode(buf) as u8;
        match discriminant {
            Self::PING_IDX => Self::Ping(Ping::decode(buf)),

            Self::KEEP_ALIVE_IDX => Self::KeepAlive(KeepAliveRequest::decode(buf)),

            Self::REGISTER_IDX => Self::Register(RegisterRequest::decode(buf)),
            Self::UPD_CHANNEL_IDX => Self::UdpChannel(UdpChannelRequest::decode(buf)),
            Self::PORT_MAPPING_IDX => Self::PortMapping(PortMappingRequest::decode(buf)),

            _ => panic!("unknown discriminant for 'RpcRequest'"),
        }
    }
}
