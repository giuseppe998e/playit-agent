mod agent;
mod control;
mod hmac;
mod socket;

use std::{
    io::{self, Result},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[async_trait::async_trait]
pub trait AsyncMessageEncode: Sized {
    async fn write_into<W>(self, buf: &mut W) -> io::Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send;
}

#[async_trait]
impl AsyncMessageEncode for u64 {
    #[inline]
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self).await
    }
}

#[async_trait]
impl<T: AsyncMessageEncode + Send> AsyncMessageEncode for Option<T> {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            Some(v) => {
                buf.write_u8(1).await?;
                v.write_into(buf).await
            }
            None => buf.write_u8(0).await,
        }
    }
}

#[async_trait]
impl AsyncMessageEncode for Vec<u8> {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.len() as _).await?;
        buf.write_all(&self).await
    }
}

#[async_trait]
impl<T: AsyncMessageEncode + Send + Sync> AsyncMessageEncode for Vec<T> {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.len() as _).await?;

        let mut bytes = Vec::with_capacity(self.len() * std::mem::size_of::<T>());
        for entry in self.into_iter() {
            entry.write_into(&mut bytes).await?;
        }

        buf.write_all(&bytes).await
    }
}

#[async_trait]
impl AsyncMessageEncode for SocketAddr {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            SocketAddr::V4(addr) => {
                buf.write_u8(4).await?;
                addr.ip().write_into(buf).await?;
                buf.write_u16(addr.port()).await
            }
            SocketAddr::V6(addr) => {
                buf.write_u8(6).await?;
                addr.ip().write_into(buf).await?;
                buf.write_u16(addr.port()).await
            }
        }
    }
}

#[async_trait]
impl AsyncMessageEncode for IpAddr {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        match self {
            IpAddr::V4(ip) => {
                buf.write_u8(4).await?;
                ip.write_into(buf).await
            }
            IpAddr::V6(ip) => {
                buf.write_u8(6).await?;
                ip.write_into(buf).await
            }
        }
    }
}

#[async_trait]
impl AsyncMessageEncode for Ipv4Addr {
    #[inline]
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_all(&self.octets()).await
    }
}

#[async_trait]
impl AsyncMessageEncode for Ipv6Addr {
    #[inline]
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_all(&self.octets()).await
    }
}
