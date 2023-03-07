use core::mem;
use std::{
    io::{self, Error, ErrorKind},
    net::SocketAddr,
};

use bytes::Buf;
use sha2::Sha256;

use super::Decode;
use crate::{
    control::{
        AgentSession, HmacSign, KeepAliveRequest, Ping, Pong, PortMappingFound, PortMappingRequest,
        PortMappingResponse, RegisterRequest, RegisterResponse, UdpChannelDetails,
        UdpChannelRequest,
    },
    socket::Socket,
    RemoteProcedureCall, RpcRequest, RpcResponse,
};

// RemoteProcedureCall
impl<T: Decode> Decode for RemoteProcedureCall<T> {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let call_id = <u64>::decode(buf)?;
        let content = T::decode(buf)?;

        Ok(Self { call_id, content })
    }
}

// RpcRequest & RpcResponse
impl Decode for RpcRequest {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u32>::decode(buf)? as u8;
        match discriminant {
            Self::PING_IDX => Ping::decode(buf).map(Self::Ping),

            Self::KEEP_ALIVE_IDX => KeepAliveRequest::decode(buf).map(Self::KeepAlive),

            Self::REGISTER_IDX => RegisterRequest::decode(buf).map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelRequest::decode(buf).map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingRequest::decode(buf).map(Self::PortMapping),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'RpcRequest'",
            )),
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

// Ping & Pong
impl Decode for Ping {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let now = <u64>::decode(buf)?;
        let session = Option::<AgentSession>::decode(buf)?;

        Ok(Ping { now, session })
    }
}

impl Decode for Pong {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() >= mem::size_of::<u64>() * 3 + mem::size_of::<u32>());
        let request_now = buf.get_u64();
        let server_now = buf.get_u64();
        let server_id = buf.get_u64();
        let data_center_id = buf.get_u32();

        let client_addr = SocketAddr::decode(buf)?;
        let tunnel_addr = SocketAddr::decode(buf)?;
        let session_expire_at = Option::<u64>::decode(buf)?;

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

// AgentSession
impl Decode for AgentSession {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() >= mem::size_of::<u64>() * 3);

        let id = buf.get_u64();
        let account_id = buf.get_u64();
        let agent_id = buf.get_u64();

        Ok(Self {
            id,
            account_id,
            agent_id,
        })
    }
}

// PortMappingRequest & PortMappingResponse
impl Decode for PortMappingRequest {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let session = AgentSession::decode(buf)?;
        let socket = Socket::decode(buf)?;

        Ok(Self { session, socket })
    }
}

impl Decode for PortMappingResponse {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let socket = Socket::decode(buf)?;
        let found = Option::<PortMappingFound>::decode(buf)?;

        Ok(Self { socket, found })
    }
}

// PortMappingFound
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

// RegisterRequest & RegisterResponse
impl Decode for RegisterRequest {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() >= mem::size_of::<u64>() * 4);
        let account_id = buf.get_u64();
        let agent_id = buf.get_u64();
        let agent_version = buf.get_u64();
        let timestamp = buf.get_u64();

        let client_addr = SocketAddr::decode(buf)?;
        let tunnel_addr = SocketAddr::decode(buf)?;
        let signature = HmacSign::<Sha256>::decode(buf)?;

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

impl Decode for RegisterResponse {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let session = AgentSession::decode(buf)?;
        let expires_at = <u64>::decode(buf)?;

        Ok(Self {
            session,
            expires_at,
        })
    }
}

// UdpChannelDetails
impl Decode for UdpChannelDetails {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let tunnel_addr = SocketAddr::decode(buf)?;

        let remaining = buf.remaining();

        super::ensure!(remaining >= mem::size_of::<u64>());
        let token_len = buf.get_u64() as usize;

        super::ensure!(remaining >= token_len);
        let token = buf.copy_to_bytes(token_len);

        Ok(Self { tunnel_addr, token })
    }
}
