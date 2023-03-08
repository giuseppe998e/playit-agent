use core::mem;
use std::{io, net::SocketAddr};

use bytes::{Buf, BufMut, Bytes};

use crate::codec::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdpChannelDetails {
    pub tunnel_addr: SocketAddr,
    pub token: Bytes,
}

impl Encode for UdpChannelDetails {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.tunnel_addr.encode(buf)?;

        crate::codec::ensure!(buf.remaining_mut() >= mem::size_of::<u64>() + self.token.len());
        buf.put_u64(self.token.len() as u64);
        buf.put(self.token);

        Ok(())
    }
}

impl Decode for UdpChannelDetails {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let tunnel_addr = SocketAddr::decode(buf)?;

        let remaining = buf.remaining();

        crate::codec::ensure!(remaining >= mem::size_of::<u64>());
        let token_len = buf.get_u64() as usize;

        crate::codec::ensure!(remaining >= token_len);
        let token = buf.copy_to_bytes(token_len);

        Ok(Self { tunnel_addr, token })
    }
}
