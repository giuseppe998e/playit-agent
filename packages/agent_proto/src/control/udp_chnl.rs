use std::net::SocketAddr;

use bytes::Bytes;

pub type UdpChannelRequest = super::agent::AgentSession;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdpChannelDetails {
    pub tunnel_addr: SocketAddr,
    pub token: Bytes,
}
