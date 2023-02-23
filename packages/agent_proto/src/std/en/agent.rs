use byteorder::{BigEndian, WriteBytesExt};

use crate::agent::AgentSession;

impl super::MessageEncode for AgentSession {
    fn write_into<W: ::std::io::Write>(self, buf: &mut W) -> ::std::io::Result<()> {
        buf.write_u64::<BigEndian>(self.id)?;
        buf.write_u64::<BigEndian>(self.account_id)?;
        buf.write_u64::<BigEndian>(self.agent_id)
    }
}
