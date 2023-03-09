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
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() >= mem::size_of::<u64>() * 3);
        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let id = <u64>::decode(buf);
        let account_id = <u64>::decode(buf);
        let agent_id = <u64>::decode(buf);

        Self {
            id,
            account_id,
            agent_id,
        }
    }
}
