use std::{
    io::{Error, ErrorKind, Result},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use async_trait::async_trait;
use tokio::io::AsyncReadExt;

use super::AsyncMessageDecode;

#[async_trait]
impl AsyncMessageDecode for u64 {
    #[inline]
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        input.read_u64().await
    }
}

#[async_trait]
impl<T: AsyncMessageDecode> AsyncMessageDecode for Option<T> {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u8().await? {
            0 => Ok(None),
            1 => T::read_from(input).await.map(Some),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"Option<T>\"."),
            )),
        }
    }
}

#[async_trait]
impl AsyncMessageDecode for Vec<u8> {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let capacity = input.read_u64().await? as usize;
        let mut vec = vec![0u8; capacity];

        input.read_exact(&mut vec).await?;
        Ok(vec)
    }
}

#[async_trait]
impl<T: AsyncMessageDecode + Send> AsyncMessageDecode for Vec<T> {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let capacity = input.read_u64().await? as usize;
        let mut vec = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            let entry = T::read_from(input).await?;
            vec.push(entry);
        }

        Ok(vec)
    }
}

#[async_trait]
impl AsyncMessageDecode for SocketAddr {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u8().await? {
            4 => Ok(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::read_from(input).await?,
                input.read_u16().await?,
            ))),
            6 => Ok(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::read_from(input).await?,
                input.read_u16().await?,
                0,
                0,
            ))),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not a \"SocketAddr\"."),
            )),
        }
    }
}

#[async_trait]
impl AsyncMessageDecode for IpAddr {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        match input.read_u8().await? {
            4 => Ipv4Addr::read_from(input).await.map(IpAddr::V4),
            6 => Ipv6Addr::read_from(input).await.map(IpAddr::V6),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not a \"IpAddr\"."),
            )),
        }
    }
}

#[async_trait]
impl AsyncMessageDecode for Ipv4Addr {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let mut bytes = [0u8; 4];

        input.read_exact(&mut bytes).await?;
        Ok(Self::from(bytes))
    }
}

#[async_trait]
impl AsyncMessageDecode for Ipv6Addr {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let mut bytes = [0u8; 16];

        input.read_exact(&mut bytes).await?;
        Ok(Self::from(bytes))
    }
}
