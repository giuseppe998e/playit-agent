use crate::{agent::AgentSession, socket::Socket};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingRequest {
    pub session: AgentSession,
    pub socket: Socket,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingResponse {
    pub socket: Socket,
    // XXX What does it means? (exist?)
    // TODO Remove Option and use "None" instead
    pub found: Option<PortMappingFound>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortMappingFound {
    ToAgent(AgentSession),
    None,
}

impl PortMappingFound {
    pub const TO_AGENT_IDX: u8 = 1;
    pub const NONE_IDX: u8 = 255;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::ToAgent(_) => Self::TO_AGENT_IDX,
            Self::None => Self::NONE_IDX,
        }
    }
}
