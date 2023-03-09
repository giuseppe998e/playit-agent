use std::io;

use bytes::{Buf, BufMut};

use crate::{
    codec::{Decode, Encode},
    rpc::common::{AgentSession, Socket},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingRequest {
    pub session: AgentSession,
    pub socket: Socket,
}

impl Decode for PortMappingRequest {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        AgentSession::check(buf)?;
        Socket::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let session = AgentSession::decode(buf);
        let socket = Socket::decode(buf);

        Self { session, socket }
    }
}

impl Encode for PortMappingRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.session.encode(buf)?;
        self.socket.encode(buf)
    }
}
