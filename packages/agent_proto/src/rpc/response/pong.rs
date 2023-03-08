use core::mem;
use std::{io, net::SocketAddr};

use bytes::{Buf, BufMut};

use crate::codec::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pong {
    pub request_now: u64,
    pub server_now: u64,
    pub server_id: u64,
    pub data_center_id: u32,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub session_expire_at: Option<u64>,
}

impl Encode for Pong {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(
            buf.remaining_mut() > mem::size_of::<u64>() * 3 + mem::size_of::<u32>()
        );
        buf.put_u64(self.request_now);
        buf.put_u64(self.server_now);
        buf.put_u64(self.server_id);
        buf.put_u32(self.data_center_id);

        self.client_addr.encode(buf)?;
        self.tunnel_addr.encode(buf)?;
        self.session_expire_at.encode(buf)
    }
}

impl Decode for Pong {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        crate::codec::ensure!(buf.remaining() >= mem::size_of::<u64>() * 3 + mem::size_of::<u32>());
        let request_now = buf.get_u64();
        let server_now = buf.get_u64();
        let server_id = buf.get_u64();
        let data_center_id = buf.get_u32();

        let client_addr = SocketAddr::decode(buf)?;
        let tunnel_addr = SocketAddr::decode(buf)?;
        let session_expire_at = Option::<u64>::decode(buf)?;

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
