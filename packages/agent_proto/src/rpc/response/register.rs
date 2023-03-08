use std::io;

use bytes::{Buf, BufMut};

use crate::{
    codec::{Decode, Encode},
    rpc::common::AgentSession,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterResponse {
    pub session: AgentSession,
    pub expires_at: u64,
}

impl Encode for RegisterResponse {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.session.encode(buf)?;
        self.expires_at.encode(buf)
    }
}

impl Decode for RegisterResponse {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let session = AgentSession::decode(buf)?;
        let expires_at = <u64>::decode(buf)?;

        Ok(Self {
            session,
            expires_at,
        })
    }
}
