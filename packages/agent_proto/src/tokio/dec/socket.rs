use std::{
    io::{Cursor, Error, ErrorKind, Result},
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
};

use async_trait::async_trait;
use tokio::io::AsyncReadExt;

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_ID_SIZE, FLOW_V4_ID,
    FLOW_V4_ID_OLD, FLOW_V6_ID,
};

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

// SocketFlow
#[async_trait]
impl AsyncMessageDecode for SocketFlow {
    /// To read a `SocketFlow` and determine whether it's a `SocketFlowV4` or `SocketFlowV6`,
    /// we need to look at the `footer_id` value, which is located after the structure's bytes.
    /// To deal with this inconvenience, we assume that the structure we're reading is
    /// `SocketFlowV4`, which has fewer bytes than `SocketFlowV6`, and we add the size of the
    /// `footer_id`. If the `footer_id` matches one of the expected values, we return the
    /// `SocketFlowV4` structure. Otherwise, we continue reading the remaining bytes
    /// to obtain the `SocketFlowV6` structure.
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let mut v4_buf = [0u8; SocketFlowV4::size() + FLOW_ID_SIZE];
        input.read_exact(&mut v4_buf).await?;

        let footer_id = {
            let mut u64_buf = [0u8; FLOW_ID_SIZE];
            let mut footer_id_buf = Cursor::new(&v4_buf[SocketFlowV4::size()..]);
            footer_id_buf.read_exact(&mut u64_buf).await?;
            u64::from_be_bytes(u64_buf)
        };

        if matches!(footer_id, FLOW_V4_ID | FLOW_V4_ID_OLD) {
            let mut v4_cursor = Cursor::new(&v4_buf);
            return SocketFlowV4::read_from(&mut v4_cursor).await.map(Self::V4);
        }

        // V6
        let mut v6_buf = [0u8; SocketFlowV6::size() - SocketFlowV4::size()];
        input.read_exact(&mut v6_buf).await?;

        let footer_id = {
            let mut u64_buf = [0u8; FLOW_ID_SIZE];
            let mut footer_id_buf =
                Cursor::new(&v6_buf[SocketFlowV6::size() - SocketFlowV4::size() - FLOW_ID_SIZE..]);
            footer_id_buf.read_exact(&mut u64_buf).await?;
            u64::from_be_bytes(u64_buf)
        };

        if matches!(footer_id, FLOW_V6_ID) {
            let mut v6_cursor = Cursor::new(&v4_buf).chain(Cursor::new(&v6_buf));
            return SocketFlowV6::read_from(&mut v6_cursor).await.map(Self::V6);
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid input for `SocketFlow`",
        ))
    }
}

#[async_trait]
impl AsyncMessageDecode for SocketFlowV4 {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let src_ip = input.read_u32().await?;
        let dest_ip = input.read_u32().await?;
        let src_port = input.read_u16().await?;
        let dest_port = input.read_u16().await?;

        let src = SocketAddrV4::new(src_ip.into(), src_port);
        let dest = SocketAddrV4::new(dest_ip.into(), dest_port);

        Ok(Self::new(src, dest))
    }
}

#[async_trait]
impl AsyncMessageDecode for SocketFlowV6 {
    async fn read_from<R>(input: &mut R) -> Result<Self>
    where
        R: AsyncReadExt + Unpin + Send,
    {
        let src_ip = input.read_u128().await?;
        let dest_ip = input.read_u128().await?;
        let src_port = input.read_u16().await?;
        let dest_port = input.read_u16().await?;
        let flowinfo = input.read_u32().await?;

        let src = SocketAddrV6::new(src_ip.into(), src_port, flowinfo, 0);
        let dest = SocketAddrV6::new(dest_ip.into(), dest_port, flowinfo, 0);

        Ok(Self::new(src, dest, flowinfo))
    }
}
