use std::net::SocketAddr;

pub type UdpChannelRequest = crate::agent::AgentSession;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdpChannelResponse {
    pub tunnel_addr: SocketAddr,
    pub token: Vec<u8>,
}
