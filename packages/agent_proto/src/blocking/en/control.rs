use std::io::{Result, Write};

use byteorder::{BigEndian, WriteBytesExt};

use crate::control::{
    ControlRequest, ControlResponse, Ping, Pong, PortMappingFound, PortMappingRequest,
    PortMappingResponse, RegisterRequest, RegisterResponse, UdpChannelDetails,
};

use super::MessageEncode;

// mod.rs
impl MessageEncode for ControlRequest {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            Self::Ping(req) => {
                buf.write_u32::<BigEndian>(Self::PING_IDX as u32)?;
                req.write_into(buf)
            }

            Self::KeepAlive(req) => {
                buf.write_u32::<BigEndian>(Self::KEEP_ALIVE_IDX as u32)?;
                req.write_into(buf)
            }

            Self::Register(req) => {
                buf.write_u32::<BigEndian>(Self::REGISTER_IDX as u32)?;
                req.write_into(buf)
            }
            Self::UdpChannel(req) => {
                buf.write_u32::<BigEndian>(Self::UPD_CHANNEL_IDX as u32)?;
                req.write_into(buf)
            }
            Self::PortMapping(req) => {
                buf.write_u32::<BigEndian>(Self::PORT_MAPPING_IDX as u32)?;
                req.write_into(buf)
            }
        }
    }
}

impl MessageEncode for ControlResponse {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            Self::Pong(resp) => {
                buf.write_u32::<BigEndian>(Self::PONG_IDX as u32)?;
                resp.write_into(buf)
            }

            Self::InvalidSignature => {
                buf.write_u32::<BigEndian>(Self::INVALID_SIGNATURE_IDX as u32)
            }
            Self::Unauthorized => buf.write_u32::<BigEndian>(Self::UNAUTHORIZED_IDX as u32),
            Self::RequestQueued => buf.write_u32::<BigEndian>(Self::REQUEST_QUEUED_IDX as u32),
            Self::TryAgainLater => buf.write_u32::<BigEndian>(Self::TRY_AGAIN_LATER_IDX as u32),

            Self::Register(resp) => {
                buf.write_u32::<BigEndian>(Self::REGISTER_IDX as u32)?;
                resp.write_into(buf)
            }
            Self::UdpChannel(resp) => {
                buf.write_u32::<BigEndian>(Self::UPD_CHANNEL_IDX as u32)?;
                resp.write_into(buf)
            }
            Self::PortMapping(resp) => {
                buf.write_u32::<BigEndian>(Self::PORT_MAPPING_IDX as u32)?;
                resp.write_into(buf)
            }
        }
    }
}

// ping.rs
impl MessageEncode for Ping {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.now)?;
        self.session.write_into(buf)
    }
}

impl MessageEncode for Pong {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.request_now)?;
        buf.write_u64::<BigEndian>(self.server_now)?;
        buf.write_u64::<BigEndian>(self.server_id)?;
        buf.write_u32::<BigEndian>(self.data_center_id)?;
        self.client_addr.write_into(buf)?;
        self.tunnel_addr.write_into(buf)?;
        self.session_expire_at.write_into(buf)
    }
}

// port_map.rs
impl MessageEncode for PortMappingRequest {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        self.session.write_into(buf)?;
        self.socket.write_into(buf)
    }
}

impl MessageEncode for PortMappingResponse {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        self.socket.write_into(buf)?;
        self.found.write_into(buf)
    }
}

impl MessageEncode for PortMappingFound {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            Self::ToAgent(resp) => {
                buf.write_u32::<BigEndian>(Self::TO_AGENT_IDX as u32)?;
                resp.write_into(buf)
            }
            Self::None => buf.write_u32::<BigEndian>(Self::NONE_IDX as u32),
        }
    }
}

// register.rs
impl MessageEncode for RegisterRequest {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.account_id)?;
        buf.write_u64::<BigEndian>(self.agent_id)?;
        buf.write_u64::<BigEndian>(self.agent_version)?;
        buf.write_u64::<BigEndian>(self.timestamp)?;
        self.client_addr.write_into(buf)?;
        self.tunnel_addr.write_into(buf)?;
        self.signature.write_into(buf)
    }
}

impl MessageEncode for RegisterResponse {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        self.session.write_into(buf)?;
        buf.write_u64::<BigEndian>(self.expires_at)
    }
}

// udp_chnl.rs
impl MessageEncode for UdpChannelDetails {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        self.tunnel_addr.write_into(buf)?;
        self.token.write_into(buf)
    }
}

// impl MessageDecode for UdpChannelRequest {
//      // NOT_NEEDED alias of "AgentSession"
// }
