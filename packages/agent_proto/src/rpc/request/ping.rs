use core::mem;
use std::io;

use bytes::{Buf, BufMut};

use crate::{
    codec::{Decode, Encode},
    rpc::common::AgentSession,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ping {
    pub now: u64,
    pub session: Option<AgentSession>,
}

impl Encode for Ping {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.now.encode(buf)?;
        self.session.encode(buf)
    }
}

impl Decode for Ping {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() > mem::size_of::<u64>());
        Option::<AgentSession>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let now = <u64>::decode(buf);
        let session = Option::<AgentSession>::decode(buf);

        Ping { now, session }
    }
}
