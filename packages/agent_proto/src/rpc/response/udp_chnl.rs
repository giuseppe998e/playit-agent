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
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        SocketAddr::check(buf)?;

        crate::codec::ensure!(buf.remaining() >= mem::size_of::<u64>());
        let token_len = <u64>::decode(buf) as usize;
        crate::codec::checked_advance!(buf.remaining() >= token_len);

        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let tunnel_addr = SocketAddr::decode(buf);

        let token_len = buf.get_u64() as usize;
        let token = buf.copy_to_bytes(token_len);

        Self { tunnel_addr, token }
    }
}
