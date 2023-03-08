mod pong;
mod port_map;
mod register;
mod udp_chnl;

use core::mem;
use std::io::{self, Error, ErrorKind};

use bytes::{Buf, BufMut};

use crate::codec::{Decode, Encode};

pub use self::{
    pong::Pong,
    port_map::{PortMappingFound, PortMappingResponse},
    register::RegisterResponse,
    udp_chnl::UdpChannelDetails,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcResponse {
    Pong(Pong),

    InvalidSignature,
    Unauthorized,
    RequestQueued,
    TryAgainLater,

    Register(RegisterResponse),
    UdpChannel(UdpChannelDetails),
    PortMapping(PortMappingResponse),
}

impl RpcResponse {
    pub const PONG_IDX: u8 = 1;

    pub const INVALID_SIGNATURE_IDX: u8 = 2;
    pub const UNAUTHORIZED_IDX: u8 = 3;
    pub const REQUEST_QUEUED_IDX: u8 = 4;
    pub const TRY_AGAIN_LATER_IDX: u8 = 5;

    pub const REGISTER_IDX: u8 = 6;
    pub const UPD_CHANNEL_IDX: u8 = 7;
    pub const PORT_MAPPING_IDX: u8 = 8;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::Pong(_) => Self::PONG_IDX,

            Self::InvalidSignature => Self::INVALID_SIGNATURE_IDX,
            Self::Unauthorized => Self::UNAUTHORIZED_IDX,
            Self::RequestQueued => Self::REQUEST_QUEUED_IDX,
            Self::TryAgainLater => Self::TRY_AGAIN_LATER_IDX,

            Self::Register(_) => Self::REGISTER_IDX,
            Self::UdpChannel(_) => Self::UPD_CHANNEL_IDX,
            Self::PortMapping(_) => Self::PORT_MAPPING_IDX,
        }
    }
}

impl Encode for RpcResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

        match self {
            Self::Pong(resp) => {
                buf.put_u32(Self::PONG_IDX as u32);
                resp.encode(buf)
            }

            Self::InvalidSignature => {
                buf.put_u32(Self::INVALID_SIGNATURE_IDX as u32);
                Ok(())
            }
            Self::Unauthorized => {
                buf.put_u32(Self::UNAUTHORIZED_IDX as u32);
                Ok(())
            }
            Self::RequestQueued => {
                buf.put_u32(Self::REQUEST_QUEUED_IDX as u32);
                Ok(())
            }
            Self::TryAgainLater => {
                buf.put_u32(Self::TRY_AGAIN_LATER_IDX as u32);
                Ok(())
            }

            Self::Register(resp) => {
                buf.put_u32(Self::REGISTER_IDX as u32);
                resp.encode(buf)
            }
            Self::UdpChannel(resp) => {
                buf.put_u32(Self::UPD_CHANNEL_IDX as u32);
                resp.encode(buf)
            }
            Self::PortMapping(resp) => {
                buf.put_u32(Self::PORT_MAPPING_IDX as u32);
                resp.encode(buf)
            }
        }
    }
}

impl Decode for RpcResponse {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u32>::decode(buf)? as u8;
        match discriminant {
            Self::PONG_IDX => Pong::decode(buf).map(Self::Pong),

            Self::INVALID_SIGNATURE_IDX => Ok(Self::InvalidSignature),
            Self::UNAUTHORIZED_IDX => Ok(Self::Unauthorized),
            Self::REQUEST_QUEUED_IDX => Ok(Self::RequestQueued),
            Self::TRY_AGAIN_LATER_IDX => Ok(Self::TryAgainLater),

            Self::REGISTER_IDX => RegisterResponse::decode(buf).map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelDetails::decode(buf).map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingResponse::decode(buf).map(Self::PortMapping),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'RpcResponse'",
            )),
        }
    }
}
