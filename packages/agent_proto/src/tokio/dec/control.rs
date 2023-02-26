use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
};

use ::tokio::io::AsyncReadExt;
use async_trait::async_trait;
use sha2::Sha256;

use crate::{
    agent::AgentSession,
    control::{
        ControlRequest, ControlResponse, KeepAliveRequest, Ping, Pong, PortMappingFound,
        PortMappingRequest, PortMappingResponse, RegisterRequest, RegisterResponse,
        UdpChannelRequest, UdpChannelDetails,
    },
    hmac::HmacSign,
    socket::Socket,
};

use super::AsyncMessageDecode;

// mod.rs
#[async_trait]
impl AsyncMessageDecode for ControlRequest {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u32().await? as u8 {
            Self::PING_IDX => Ping::read_from(input).await.map(Self::Ping),

            Self::KEEP_ALIVE_IDX => KeepAliveRequest::read_from(input)
                .await
                .map(Self::KeepAlive),

            Self::REGISTER_IDX => RegisterRequest::read_from(input).await.map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelRequest::read_from(input)
                .await
                .map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingRequest::read_from(input)
                .await
                .map(Self::PortMapping),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlRequest\"."),
            )),
        }
    }
}

#[async_trait]
impl AsyncMessageDecode for ControlResponse {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u32().await? as u8 {
            Self::PONG_IDX => Pong::read_from(input).await.map(Self::Pong),

            Self::INVALID_SIGNATURE_IDX => Ok(Self::InvalidSignature),
            Self::UNAUTHORIZED_IDX => Ok(Self::Unauthorized),
            Self::REQUEST_QUEUED_IDX => Ok(Self::RequestQueued),
            Self::TRY_AGAIN_LATER_IDX => Ok(Self::TryAgainLater),

            Self::REGISTER_IDX => RegisterResponse::read_from(input).await.map(Self::Register),
            Self::UPD_CHANNEL_IDX => UdpChannelDetails::read_from(input)
                .await
                .map(Self::UdpChannel),
            Self::PORT_MAPPING_IDX => PortMappingResponse::read_from(input)
                .await
                .map(Self::PortMapping),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlResponse\"."),
            )),
        }
    }
}

// ping.rs
#[async_trait]
impl AsyncMessageDecode for Ping {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let now = input.read_u64().await?;
        let session = Option::<AgentSession>::read_from(input).await?;

        Ok(Ping { now, session })
    }
}

#[async_trait]
impl AsyncMessageDecode for Pong {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let request_now = input.read_u64().await?;
        let server_now = input.read_u64().await?;
        let server_id = input.read_u64().await?;
        let data_center_id = input.read_u32().await?;
        let client_addr = SocketAddr::read_from(input).await?;
        let tunnel_addr = SocketAddr::read_from(input).await?;
        let session_expire_at = Option::read_from(input).await?;

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
#[async_trait]
impl AsyncMessageDecode for PortMappingRequest {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let session = AgentSession::read_from(input).await?;
        let socket = Socket::read_from(input).await?;

        Ok(Self { session, socket })
    }
}

#[async_trait]
impl AsyncMessageDecode for PortMappingResponse {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let socket = Socket::read_from(input).await?;
        let found = Option::<PortMappingFound>::read_from(input).await?;

        Ok(Self { socket, found })
    }
}

#[async_trait]
impl AsyncMessageDecode for PortMappingFound {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u32().await? as u8 {
            Self::TO_AGENT_IDX => AgentSession::read_from(input).await.map(Self::ToAgent),
            Self::NONE_IDX => Ok(Self::None),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"control::ControlResponse\"."),
            )),
        }
    }
}

// register.rs
#[async_trait]
impl AsyncMessageDecode for RegisterRequest {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let account_id = input.read_u64().await?;
        let agent_id = input.read_u64().await?;
        let agent_version = input.read_u64().await?;
        let timestamp = input.read_u64().await?;
        let client_addr = SocketAddr::read_from(input).await?;
        let tunnel_addr = SocketAddr::read_from(input).await?;
        let signature = HmacSign::<Sha256>::read_from(input).await?;

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

#[async_trait]
impl AsyncMessageDecode for RegisterResponse {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let session = AgentSession::read_from(input).await?;
        let expires_at = input.read_u64().await?;

        Ok(Self {
            session,
            expires_at,
        })
    }
}

// udp_chnl.rs
#[async_trait]
impl AsyncMessageDecode for UdpChannelDetails {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let tunnel_addr = SocketAddr::read_from(input).await?;
        let token = Vec::<u8>::read_from(input).await?;

        Ok(Self { tunnel_addr, token })
    }
}

// #[async_trait]
// impl AsyncMessageDecode for UdpChannelRequest {
//      // NOT_NEEDED alias of "AgentSession"
// }
