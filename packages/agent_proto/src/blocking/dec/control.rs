use std::{
    io::{Error, ErrorKind, Read, Result},
    net::SocketAddr,
};

use byteorder::{BigEndian, ReadBytesExt};
use sha2::Sha256;

use crate::{
    agent::AgentSession,
    control::{
        ControlRequest, ControlResponse, KeepAliveRequest, Ping, Pong, PortMappingFound,
        PortMappingRequest, PortMappingResponse, RegisterRequest, RegisterResponse,
        UdpChannelDetails, UdpChannelRequest,
    },
    hmac::HmacSign,
    socket::Socket,
};

use super::MessageDecode;

// mod.rs
impl MessageDecode for ControlRequest {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        match input.read_u32::<BigEndian>()? as u8 {
            Self::PING_IDX => Ping::read_from(input).map(Self::Ping),

            Self::KEEP_ALIVE_IDX => KeepAliveRequest::read_from(input).map(Self::KeepAlive),

            Self::REGISTER_IDX => RegisterRequest::read_from(input).map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelRequest::read_from(input).map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingRequest::read_from(input).map(Self::PortMapping),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlRequest\"."),
            )),
        }
    }
}

impl MessageDecode for ControlResponse {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        match input.read_u32::<BigEndian>()? as u8 {
            Self::PONG_IDX => Pong::read_from(input).map(Self::Pong),

            Self::INVALID_SIGNATURE_IDX => Ok(Self::InvalidSignature),
            Self::UNAUTHORIZED_IDX => Ok(Self::Unauthorized),
            Self::REQUEST_QUEUED_IDX => Ok(Self::RequestQueued),
            Self::TRY_AGAIN_LATER_IDX => Ok(Self::TryAgainLater),

            Self::REGISTER_IDX => RegisterResponse::read_from(input).map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelDetails::read_from(input).map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingResponse::read_from(input).map(Self::PortMapping),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlResponse\"."),
            )),
        }
    }
}

// ping.rs
impl MessageDecode for Ping {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let now = input.read_u64::<BigEndian>()?;
        let session = Option::<AgentSession>::read_from(input)?;

        Ok(Ping { now, session })
    }
}

impl MessageDecode for Pong {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let request_now = input.read_u64::<BigEndian>()?;
        let server_now = input.read_u64::<BigEndian>()?;
        let server_id = input.read_u64::<BigEndian>()?;
        let data_center_id = input.read_u32::<BigEndian>()?;
        let client_addr = SocketAddr::read_from(input)?;
        let tunnel_addr = SocketAddr::read_from(input)?;
        let session_expire_at = Option::read_from(input)?;

        Ok(Self {
            request_now,
            server_now,
            server_id,
            data_center_id,
            client_addr,
            tunnel_addr,
            session_expire_at,
        })
    }
}

// port_map.rs
impl MessageDecode for PortMappingRequest {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let session = AgentSession::read_from(input)?;
        let socket = Socket::read_from(input)?;

        Ok(Self { session, socket })
    }
}

impl MessageDecode for PortMappingResponse {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let socket = Socket::read_from(input)?;
        let found = Option::<PortMappingFound>::read_from(input)?;

        Ok(Self { socket, found })
    }
}

impl MessageDecode for PortMappingFound {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        match input.read_u32::<BigEndian>()? as u8 {
            Self::TO_AGENT_IDX => AgentSession::read_from(input).map(Self::ToAgent),
            Self::NONE_IDX => Ok(Self::None),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlResponse\"."),
            )),
        }
    }
}

// register.rs
impl MessageDecode for RegisterRequest {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let account_id = input.read_u64::<BigEndian>()?;
        let agent_id = input.read_u64::<BigEndian>()?;
        let agent_version = input.read_u64::<BigEndian>()?;
        let timestamp = input.read_u64::<BigEndian>()?;
        let client_addr = SocketAddr::read_from(input)?;
        let tunnel_addr = SocketAddr::read_from(input)?;
        let signature = HmacSign::<Sha256>::read_from(input)?;

        Ok(Self {
            account_id,
            agent_id,
            agent_version,
            timestamp,
            client_addr,
            tunnel_addr,
            signature,
        })
    }
}

impl MessageDecode for RegisterResponse {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let session = AgentSession::read_from(input)?;
        let expires_at = input.read_u64::<BigEndian>()?;

        Ok(Self {
            session,
            expires_at,
        })
    }
}

// udp_chnl.rs
impl MessageDecode for UdpChannelDetails {
    fn read_from<R: Read + ?Sized>(input: &mut R) -> Result<Self> {
        let tunnel_addr = SocketAddr::read_from(input)?;
        let token = Vec::<u8>::read_from(input)?;

        Ok(Self { tunnel_addr, token })
    }
}

// impl MessageDecode for UdpChannelRequest {
//      // NOT_NEEDED alias of "AgentSession"
// }
