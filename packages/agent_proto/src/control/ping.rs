use std::net::SocketAddr;

use super::agent::AgentSession;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ping {
    pub now: u64,
    pub session: Option<AgentSession>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pong {
    pub request_now: u64,
    pub server_now: u64,
    pub server_id: u64,
    pub data_center_id: u32,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub session_expire_at: Option<u64>,
}
