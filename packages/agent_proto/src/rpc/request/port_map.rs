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
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let session = AgentSession::decode(buf)?;
        let socket = Socket::decode(buf)?;

        Ok(Self { session, socket })
    }
}

impl Encode for PortMappingRequest {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.session.encode(buf)?;
        self.socket.encode(buf)
    }
}
