use super::agent::AgentSession;
use crate::socket::Socket;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingRequest {
    pub session: AgentSession,
    pub socket: Socket,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortMappingResponse {
    pub socket: Socket,
    // FIXME Remove Option and use "None" instead (Needs server-side edit)
    // What does it means? (exist?)
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
