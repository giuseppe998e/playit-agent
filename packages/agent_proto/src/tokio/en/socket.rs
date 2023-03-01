use std::io::Result;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_V4_ID_OLD,
    FLOW_V6_ID,
};

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

// SocketFlow
#[async_trait]
impl AsyncMessageEncode for SocketFlow {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        match self {
            SocketFlow::V4(flow) => {
                flow.write_into(buf).await?;
                buf.write_u64(FLOW_V4_ID_OLD).await
            }
            SocketFlow::V6(flow) => {
                flow.write_into(buf).await?;
                buf.write_u64(FLOW_V6_ID).await
            }
        }
    }
}

#[async_trait]
impl AsyncMessageEncode for SocketFlowV4 {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        let src = self.src();
        let dest = self.dest();

        buf.write_u32((*src.ip()).into()).await?;
        buf.write_u32((*dest.ip()).into()).await?;
        buf.write_u16(src.port()).await?;
        buf.write_u16(dest.port()).await
    }
}

#[async_trait]
impl AsyncMessageEncode for SocketFlowV6 {
    async fn write_into<W>(self, buf: &mut W) -> Result<()>
    where
        W: AsyncWriteExt + Unpin + Send,
    {
        let src = self.src();
        let dest = self.dest();
        let flowinfo = self.flowinfo();

        buf.write_u128((*src.ip()).into()).await?;
        buf.write_u128((*dest.ip()).into()).await?;
        buf.write_u16(src.port()).await?;
        buf.write_u16(dest.port()).await?;
        buf.write_u32(flowinfo).await
    }
}
