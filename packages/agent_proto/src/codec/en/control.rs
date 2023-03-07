use core::mem;
use std::io;

use bytes::BufMut;

use super::Encode;
use crate::{
    control::{
        AgentSession, Ping, Pong, PortMappingFound, PortMappingRequest, PortMappingResponse,
        RegisterRequest, RegisterResponse, UdpChannelDetails,
    },
    RemoteProcedureCall, RpcRequest, RpcResponse,
};

// RemoteProcedureCall
impl<T: Encode> Encode for RemoteProcedureCall<T> {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u64>());
        buf.put_u64(self.call_id);
        self.content.encode(buf)
    }
}

// RpcRequest & RpcResponse
impl Encode for RpcRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

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

impl Encode for RpcResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

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

// Ping
impl Encode for Ping {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.now.encode(buf)?;
        self.session.encode(buf)
    }
}

impl Encode for Pong {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u64>() * 3 + mem::size_of::<u32>());
        buf.put_u64(self.request_now);
        buf.put_u64(self.server_now);
        buf.put_u64(self.server_id);
        buf.put_u32(self.data_center_id);

        self.client_addr.encode(buf)?;
        self.tunnel_addr.encode(buf)?;
        self.session_expire_at.encode(buf)
    }
}

// AgentSession
impl Encode for AgentSession {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() >= mem::size_of::<u64>() * 3);

        buf.put_u64(self.id);
        buf.put_u64(self.account_id);
        buf.put_u64(self.agent_id);

        Ok(())
    }
}

// PortMappingRequest & PortMappingResponse
impl Encode for PortMappingRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.session.encode(buf)?;
        self.socket.encode(buf)
    }
}

impl Encode for PortMappingResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.socket.encode(buf)?;
        self.found.encode(buf)
    }
}

// PortMappingFound
impl Encode for PortMappingFound {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u32>());

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

// RegisterRequest & RegisterResponse
impl Encode for RegisterRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u64>() * 4);
        buf.put_u64(self.account_id);
        buf.put_u64(self.agent_id);
        buf.put_u64(self.agent_version);
        buf.put_u64(self.timestamp);

        self.client_addr.encode(buf)?;
        self.tunnel_addr.encode(buf)?;
        self.signature.encode(buf)
    }
}

impl Encode for RegisterResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.session.encode(buf)?;
        self.expires_at.encode(buf)
    }
}

// UdpChannelDetails
impl Encode for UdpChannelDetails {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.tunnel_addr.encode(buf)?;

        super::ensure!(buf.remaining_mut() >= mem::size_of::<u64>() + self.token.len());
        buf.put_u64(self.token.len() as u64);
        buf.put(self.token);

        Ok(())
    }
}
