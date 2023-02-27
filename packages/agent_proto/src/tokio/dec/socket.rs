use std::{
    io::{Cursor, Error, ErrorKind, Result},
    mem,
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
};

use async_trait::async_trait;
use tokio::io::AsyncReadExt;

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, V4_FOOTER_ID, V4_FOOTER_ID_OLD,
    V4_LEN, V6_FOOTER_ID, V6_LEN,
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
        // Initial length of buffer to read
        const INIT_LEN: usize = V4_LEN + mem::size_of::<u64>();

        // Initialize a buffer to hold the input bytes
        let mut input_buf = Vec::<u8>::new();

        // Parse the `SocketFlowV4` variant
        // Read `SocketFlowV4` structure plus `footer_id` (20 bytes)
        input_buf.resize(INIT_LEN, 0);
        input.read_exact(&mut input_buf).await?;

        // Parse `footer_id`
        let mut footer_id_bytes = &input_buf[V4_LEN..];
        let footer_id = footer_id_bytes.read_u64().await?;

        // Check and parse `SocketFlowV4`
        if matches!(footer_id, V4_FOOTER_ID | V4_FOOTER_ID_OLD) {
            let mut v4_cursor = Cursor::new(&input_buf[..V4_LEN]);
            return SocketFlowV4::read_from(&mut v4_cursor).await.map(Self::V4);
        }

        // If `footer_id` did not match any `SocketFlowV4` variant,
        // parse the `SocketFlowV6` variant
        // Read `SocketFlowV6` structure plus `footer_id` (48 bytes)
        input_buf.resize(INIT_LEN + V6_LEN - V4_LEN, 0);
        input.read_exact(&mut input_buf[INIT_LEN..]).await?;

        // Parse `footer_id`
        let mut footer_id_bytes = &input_buf[V6_LEN..];
        let footer_id = footer_id_bytes.read_u64().await?;

        // Check and parse `SocketFlowV6`
        if matches!(footer_id, V6_FOOTER_ID) {
            let mut v6_cursor = Cursor::new(&input_buf[..V6_LEN]);
            return SocketFlowV6::read_from(&mut v6_cursor).await.map(Self::V6);
        }

        // If `footer_id` did not match any `SocketFlow` variant, return an error
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
