use std::io::Result;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::socket::{Port, Protocol, Socket};

use super::AsyncMessageEncode;

#[async_trait]
impl AsyncMessageEncode for Socket {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        self.ip.write_into(buf).await?;
        self.port.write_into(buf).await?;
        self.proto.write_into(buf).await
    }
}

#[async_trait]
impl AsyncMessageEncode for Port {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        let (start, end) = match self {
            Self::Single(port) => (port, port),
            Self::Range(range) => (*range.start(), *range.end()),
        };

        buf.write_u16(start).await?;
        buf.write_u16(end).await
    }
}

#[async_trait]
impl AsyncMessageEncode for Protocol {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        buf.write_u8(self.into()).await
    }
}
