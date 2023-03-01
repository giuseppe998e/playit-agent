use byteorder::{BigEndian, ReadBytesExt};

use crate::agent::AgentSession;

impl super::MessageDecode for AgentSession {
    fn read_from<R: ::std::io::Read + ?Sized>(input: &mut R) -> ::std::io::Result<Self> {
        let id = input.read_u64::<BigEndian>()?;
        let account_id = input.read_u64::<BigEndian>()?;
        let agent_id = input.read_u64::<BigEndian>()?;

        Ok(Self {
            id,
            account_id,
            agent_id,
        })
    }
}
