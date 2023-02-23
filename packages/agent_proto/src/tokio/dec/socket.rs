use std::{
    io::{Error, ErrorKind, Result},
    net::IpAddr,
};

use async_trait::async_trait;
use tokio::io::AsyncReadExt;

use crate::socket::{Port, Protocol, Socket};

use super::AsyncMessageDecode;

#[async_trait]
impl AsyncMessageDecode for Socket {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let ip = IpAddr::read_from(input).await?;
        let port = Port::read_from(input).await?;
        let proto = Protocol::read_from(input).await?;

        Ok(Self { ip, port, proto })
    }
}

#[async_trait]
impl AsyncMessageDecode for Port {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let start = input.read_u16().await?;
        let end = input.read_u16().await?;

        match start == end {
            true => Ok(Self::Single(start)),
            false => Ok(Self::Range(start..=end)),
        }
    }
}

#[async_trait]
impl AsyncMessageDecode for Protocol {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u8().await? {
            1 => Ok(Self::Tcp),
            2 => Ok(Self::Udp),
            3 => Ok(Self::Both),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"socket::Protocol\"."),
            )),
        }
    }
}
