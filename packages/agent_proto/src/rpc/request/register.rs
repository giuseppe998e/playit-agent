use core::mem;
use std::{io, net::SocketAddr};

use bytes::{Buf, BufMut};
use sha2::Sha256;

use crate::{
    codec::{Decode, Encode},
    hmac::HmacSign,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterRequest {
    pub account_id: u64,
    pub agent_id: u64,
    pub agent_version: u64,
    pub timestamp: u64,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub signature: HmacSign<Sha256>,
}

// XXX Methods not implemented due to not being used
// https://github.com/playit-cloud/playit-agent/blob/185bfb0a75d3af75d915c84fa1bcd514653220f1/packages/agent_proto/src/control_messages.rs#L111

impl Encode for RegisterRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > mem::size_of::<u64>() * 4);
        buf.put_u64(self.account_id);
        buf.put_u64(self.agent_id);
        buf.put_u64(self.agent_version);
        buf.put_u64(self.timestamp);

        self.client_addr.encode(buf)?;
        self.tunnel_addr.encode(buf)?;
        self.signature.encode(buf)
    }
}

impl Decode for RegisterRequest {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() > mem::size_of::<u64>() * 4);
        SocketAddr::check(buf)?;
        SocketAddr::check(buf)?;
        HmacSign::<Sha256>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let account_id = <u64>::decode(buf);
        let agent_id = <u64>::decode(buf);
        let agent_version = <u64>::decode(buf);
        let timestamp = <u64>::decode(buf);

        let client_addr = SocketAddr::decode(buf);
        let tunnel_addr = SocketAddr::decode(buf);
        let signature = HmacSign::<Sha256>::decode(buf);

        Self {
            account_id,
            agent_id,
            agent_version,
            timestamp,
            client_addr,
            tunnel_addr,
            signature,
        }
    }
}
