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

// TODO impl

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterResponse {
    pub session: AgentSession,
    pub expires_at: u64,
}
