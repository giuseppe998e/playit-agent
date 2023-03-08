use core::mem;
use std::io::{self, Error, ErrorKind};

use bytes::{Buf, BufMut};

use crate::{
    codec::{Decode, Encode},
    rpc::common::{AgentSession, Socket},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingResponse {
    pub socket: Socket,
    // FIXME Remove Option and use "None" instead (Needs server-side edit)
    // What does it means? (exist?)
    pub found: Option<PortMappingFound>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortMappingFound {
    ToAgent(AgentSession),
    None,
}

impl Encode for PortMappingResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.socket.encode(buf)?;
        self.found.encode(buf)
    }
}

impl Decode for PortMappingResponse {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let socket = Socket::decode(buf)?;
        let found = Option::<PortMappingFound>::decode(buf)?;

        Ok(Self { socket, found })
    }
}

impl PortMappingFound {
    pub const TO_AGENT_IDX: u8 = 1;
    pub const NONE_IDX: u8 = 255;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::ToAgent(_) => Self::TO_AGENT_IDX,
            Self::None => Self::NONE_IDX,
        }
    }
}

impl Encode for PortMappingFound {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

        match self {
            Self::ToAgent(resp) => {
                buf.put_u32(Self::TO_AGENT_IDX as u32);
                resp.encode(buf)
            }
            Self::None => {
                buf.put_u32(Self::NONE_IDX as u32);
                Ok(())
            }
        }
    }
}

impl Decode for PortMappingFound {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u32>::decode(buf)? as u8;
        match discriminant {
            Self::TO_AGENT_IDX => AgentSession::decode(buf).map(Self::ToAgent),
            Self::NONE_IDX => Ok(Self::None),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'PortMappingFound'",
            )),
        }
    }
}
