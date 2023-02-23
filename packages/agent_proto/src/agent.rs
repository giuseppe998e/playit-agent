#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentSession {
    pub id: u64,
    pub account_id: u64,
    pub agent_id: u64,
}
