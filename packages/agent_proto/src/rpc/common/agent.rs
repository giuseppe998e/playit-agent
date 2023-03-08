use core::mem;
use std::io;

use bytes::{Buf, BufMut};

use crate::codec::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentSession {
    pub id: u64,
    pub account_id: u64,
    pub agent_id: u64,
}

impl Encode for AgentSession {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() >= mem::size_of::<u64>() * 3);

        buf.put_u64(self.id);
        buf.put_u64(self.account_id);
        buf.put_u64(self.agent_id);

        Ok(())
    }
}

impl Decode for AgentSession {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        crate::codec::ensure!(buf.remaining() >= mem::size_of::<u64>() * 3);

        let id = buf.get_u64();
        let account_id = buf.get_u64();
        let agent_id = buf.get_u64();

        Ok(Self {
            id,
            account_id,
            agent_id,
        })
    }
}
