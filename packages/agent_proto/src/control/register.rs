use std::net::SocketAddr;

use sha2::Sha256;

use crate::{agent::AgentSession, hmac::HmacSign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterRequest {
    pub account_id: u64,
    pub agent_id: u64,
    pub agent_version: u64,
    pub timestamp: u64,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub signature: HmacSign<Sha256>,
}

// XXX Methods not implemented due to not being used
// https://github.com/playit-cloud/playit-agent/blob/185bfb0a75d3af75d915c84fa1bcd514653220f1/packages/agent_proto/src/control_messages.rs#L111 

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterResponse {
    pub session: AgentSession,
    pub expires_at: u64,
}
