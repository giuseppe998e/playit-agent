use std::io::Result;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::control::{
    ControlRequest, ControlResponse, Ping, Pong, PortMappingFound, PortMappingRequest,
    PortMappingResponse, RegisterRequest, RegisterResponse, RemoteProcedureCall, UdpChannelDetails,
};

use super::AsyncMessageEncode;

// mod.rs
#[async_trait]
impl<T: AsyncMessageEncode + Send> AsyncMessageEncode for RemoteProcedureCall<T> {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.request_id).await?;
        self.content.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for ControlRequest {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            Self::Ping(req) => {
                buf.write_u32(Self::PING_IDX as u32).await?;
                req.write_into(buf).await
            }

            Self::KeepAlive(req) => {
                buf.write_u32(Self::KEEP_ALIVE_IDX as u32).await?;
                req.write_into(buf).await
            }

            Self::Register(req) => {
                buf.write_u32(Self::REGISTER_IDX as u32).await?;
                req.write_into(buf).await
            }
            Self::UdpChannel(req) => {
                buf.write_u32(Self::UPD_CHANNEL_IDX as u32).await?;
                req.write_into(buf).await
            }
            Self::PortMapping(req) => {
                buf.write_u32(Self::PORT_MAPPING_IDX as u32).await?;
                req.write_into(buf).await
            }
        }
    }
}

#[async_trait]
impl AsyncMessageEncode for ControlResponse {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            Self::Pong(resp) => {
                buf.write_u32(Self::PONG_IDX as u32).await?;
                resp.write_into(buf).await
            }

            Self::InvalidSignature => buf.write_u32(Self::INVALID_SIGNATURE_IDX as u32).await,
            Self::Unauthorized => buf.write_u32(Self::UNAUTHORIZED_IDX as u32).await,
            Self::RequestQueued => buf.write_u32(Self::REQUEST_QUEUED_IDX as u32).await,
            Self::TryAgainLater => buf.write_u32(Self::TRY_AGAIN_LATER_IDX as u32).await,

            Self::Register(resp) => {
                buf.write_u32(Self::REGISTER_IDX as u32).await?;
                resp.write_into(buf).await
            }
            Self::UdpChannel(resp) => {
                buf.write_u32(Self::UPD_CHANNEL_IDX as u32).await?;
                resp.write_into(buf).await
            }
            Self::PortMapping(resp) => {
                buf.write_u32(Self::PORT_MAPPING_IDX as u32).await?;
                resp.write_into(buf).await
            }
        }
    }
}

// ping.rs
#[async_trait]
impl AsyncMessageEncode for Ping {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.now).await?;
        self.session.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for Pong {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.request_now).await?;
        buf.write_u64(self.server_now).await?;
        buf.write_u64(self.server_id).await?;
        buf.write_u32(self.data_center_id).await?;
        self.client_addr.write_into(buf).await?;
        self.tunnel_addr.write_into(buf).await?;
        self.session_expire_at.write_into(buf).await
    }
}

// port_map.rs
#[async_trait]
impl AsyncMessageEncode for PortMappingRequest {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        self.session.write_into(buf).await?;
        self.socket.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for PortMappingResponse {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        self.socket.write_into(buf).await?;
        self.found.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for PortMappingFound {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            Self::ToAgent(resp) => {
                buf.write_u32(Self::TO_AGENT_IDX as u32).await?;
                resp.write_into(buf).await
            }
            Self::None => buf.write_u32(Self::NONE_IDX as u32).await,
        }
    }
}

// register.rs
#[async_trait]
impl AsyncMessageEncode for RegisterRequest {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.account_id).await?;
        buf.write_u64(self.agent_id).await?;
        buf.write_u64(self.agent_version).await?;
        buf.write_u64(self.timestamp).await?;
        self.client_addr.write_into(buf).await?;
        self.tunnel_addr.write_into(buf).await?;
        self.signature.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for RegisterResponse {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        self.session.write_into(buf).await?;
        buf.write_u64(self.expires_at).await
    }
}

// udp_chnl.rs
#[async_trait]
impl AsyncMessageEncode for UdpChannelDetails {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        self.tunnel_addr.write_into(buf).await?;
        self.token.write_into(buf).await
    }
}

// #[async_trait]
// impl AsyncMessageEncode for UdpChannelRequest {
//      // NOT_NEEDED alias of "AgentSession"
// }
