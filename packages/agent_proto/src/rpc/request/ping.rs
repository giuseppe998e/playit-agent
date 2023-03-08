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
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let now = <u64>::decode(buf)?;
        let session = Option::<AgentSession>::decode(buf)?;

        Ok(Ping { now, session })
    }
}
