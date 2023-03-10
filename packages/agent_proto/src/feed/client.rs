use core::mem;
use std::{io, net::SocketAddr};

use bytes::{Buf, BufMut};

use crate::{Decode, Encode};

pub type ClaimInstructions = crate::rpc::response::UdpChannelDetails;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientDetails {
    pub connect_addr: SocketAddr,
    pub peer_addr: SocketAddr,
    pub claim_instructions: ClaimInstructions,
    pub tunnel_id: u64,
    pub data_center_id: u32,
}

impl Encode for ClientDetails {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.connect_addr.encode(buf)?;
        self.peer_addr.encode(buf)?;
        self.claim_instructions.encode(buf)?;

        crate::codec::ensure!(buf.remaining_mut() >= mem::size_of::<u64>() + mem::size_of::<u32>());
        buf.put_u64(self.tunnel_id);
        buf.put_u32(self.data_center_id);
        Ok(())
    }
}

impl Decode for ClientDetails {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        SocketAddr::check(buf)?;
        SocketAddr::check(buf)?;
        ClaimInstructions::check(buf)?;

        crate::codec::checked_advance!(
            buf.remaining() >= mem::size_of::<u64>() + mem::size_of::<u32>()
        );

        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let connect_addr = SocketAddr::decode(buf);
        let peer_addr = SocketAddr::decode(buf);
        let claim_instructions = ClaimInstructions::decode(buf);
        let tunnel_id = <u64>::decode(buf);
        let data_center_id = <u32>::decode(buf);

        Self {
            connect_addr,
            peer_addr,
            claim_instructions,
            tunnel_id,
            data_center_id,
        }
    }
}
