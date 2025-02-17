use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{AgentSessionId, PortRange};
use crate::encoding::MessageEncoding;
use crate::hmac::HmacSha256;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ControlRequest {
    Ping(Ping),
    AgentRegister(AgentRegister),
    AgentKeepAlive(AgentSessionId),
    SetupUdpChannel(AgentSessionId),
    AgentCheckPortMapping(AgentCheckPortMapping),
}

impl MessageEncoding for ControlRequest {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        match self {
            ControlRequest::Ping(ping) => {
                out.write_u32::<BigEndian>(1)?;
                ping.write_to(out)
            }
            ControlRequest::AgentRegister(register) => {
                out.write_u32::<BigEndian>(2)?;
                register.write_to(out)
            }
            ControlRequest::AgentKeepAlive(id) => {
                out.write_u32::<BigEndian>(3)?;
                id.write_to(out)
            }
            ControlRequest::SetupUdpChannel(id) => {
                out.write_u32::<BigEndian>(4)?;
                id.write_to(out)
            }
            ControlRequest::AgentCheckPortMapping(check) => {
                out.write_u32::<BigEndian>(5)?;
                check.write_to(out)
            }
        }
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        match read.read_u32::<BigEndian>()? {
            1 => Ok(ControlRequest::Ping(Ping::read_from(read)?)),
            2 => Ok(ControlRequest::AgentRegister(AgentRegister::read_from(read)?)),
            3 => Ok(ControlRequest::AgentKeepAlive(AgentSessionId::read_from(read)?)),
            4 => Ok(ControlRequest::SetupUdpChannel(AgentSessionId::read_from(read)?)),
            5 => Ok(ControlRequest::AgentCheckPortMapping(AgentCheckPortMapping::read_from(read)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid ControlRequest id")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AgentCheckPortMapping {
    pub agent_session_id: AgentSessionId,
    pub port_range: PortRange,
}

impl MessageEncoding for AgentCheckPortMapping {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        self.agent_session_id.write_to(out)?;
        self.port_range.write_to(out)
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(AgentCheckPortMapping {
            agent_session_id: AgentSessionId::read_from(read)?,
            port_range: PortRange::read_from(read)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Ping {
    pub now: u64,
    pub session_id: Option<AgentSessionId>,
}

impl MessageEncoding for Ping {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        out.write_u64::<BigEndian>(self.now)?;
        self.session_id.write_to(out)
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(Ping {
            now: read.read_u64::<BigEndian>()?,
            session_id: Option::read_from(read)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AgentRegister {
    pub account_id: u64,
    pub agent_id: u64,
    pub agent_version: u64,
    pub timestamp: u64,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub signature: [u8; 32],
}

impl AgentRegister {
    pub fn update_signature(&mut self, temp_buffer: &mut Vec<u8>, hmac: &HmacSha256) {
        self.write_plain(temp_buffer);
        self.signature = hmac.sign(&temp_buffer);
    }

    pub fn verify_signature(&self, temp_buffer: &mut Vec<u8>, hmac: &HmacSha256) -> bool {
        self.write_plain(temp_buffer);
        hmac.verify(&temp_buffer, &self.signature).is_ok()
    }

    fn write_plain(&self, temp_buffer: &mut Vec<u8>) {
        temp_buffer.clear();
        temp_buffer.write_u64::<BigEndian>(self.account_id).unwrap();
        temp_buffer.write_u64::<BigEndian>(self.agent_id).unwrap();
        temp_buffer.write_u64::<BigEndian>(self.agent_version).unwrap();
        temp_buffer.write_u64::<BigEndian>(self.timestamp).unwrap();
        self.client_addr.write_to(temp_buffer).unwrap();
        self.tunnel_addr.write_to(temp_buffer).unwrap();
    }
}

impl MessageEncoding for AgentRegister {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        out.write_u64::<BigEndian>(self.account_id)?;
        out.write_u64::<BigEndian>(self.agent_id)?;
        out.write_u64::<BigEndian>(self.agent_version)?;
        out.write_u64::<BigEndian>(self.timestamp)?;
        self.client_addr.write_to(out)?;
        self.tunnel_addr.write_to(out)?;
        if out.write(&self.signature)? != 32 {
            return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "failed to write full signature"));
        }
        Ok(())
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        let mut res = AgentRegister {
            account_id: read.read_u64::<BigEndian>()?,
            agent_id: read.read_u64::<BigEndian>()?,
            agent_version: read.read_u64::<BigEndian>()?,
            timestamp: read.read_u64::<BigEndian>()?,
            client_addr: SocketAddr::read_from(read)?,
            tunnel_addr: SocketAddr::read_from(read)?,
            signature: [0u8; 32],
        };

        if read.read(&mut res.signature[..])? != 32 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "missing signature"));
        }

        Ok(res)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ControlResponse {
    Pong(Pong),
    InvalidSignature,
    Unauthorized,
    RequestQueued,
    TryAgainLater,
    AgentRegistered(AgentRegistered),
    AgentPortMapping(AgentPortMapping),
    UdpChannelDetails(UdpChannelDetails),
}

impl MessageEncoding for ControlResponse {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        match self {
            ControlResponse::Pong(pong) => {
                out.write_u32::<BigEndian>(1)?;
                pong.write_to(out)
            }
            ControlResponse::InvalidSignature => out.write_u32::<BigEndian>(2),
            ControlResponse::Unauthorized => out.write_u32::<BigEndian>(3),
            ControlResponse::RequestQueued => out.write_u32::<BigEndian>(4),
            ControlResponse::TryAgainLater => out.write_u32::<BigEndian>(5),
            ControlResponse::AgentRegistered(registered) => {
                out.write_u32::<BigEndian>(6)?;
                registered.write_to(out)
            }
            ControlResponse::AgentPortMapping(mapping) => {
                out.write_u32::<BigEndian>(7)?;
                mapping.write_to(out)
            }
            ControlResponse::UdpChannelDetails(details) => {
                out.write_u32::<BigEndian>(8)?;
                details.write_to(out)
            }
        }
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        match read.read_u32::<BigEndian>()? {
            1 => Ok(ControlResponse::Pong(Pong::read_from(read)?)),
            2 => Ok(ControlResponse::InvalidSignature),
            3 => Ok(ControlResponse::Unauthorized),
            4 => Ok(ControlResponse::RequestQueued),
            5 => Ok(ControlResponse::TryAgainLater),
            6 => Ok(ControlResponse::AgentRegistered(AgentRegistered::read_from(read)?)),
            7 => Ok(ControlResponse::AgentPortMapping(AgentPortMapping::read_from(read)?)),
            8 => Ok(ControlResponse::UdpChannelDetails(UdpChannelDetails::read_from(read)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid ControlResponse id")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AgentPortMapping {
    pub range: PortRange,
    pub found: Option<AgentPortMappingFound>,
}

impl MessageEncoding for AgentPortMapping {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        self.range.write_to(out)?;
        self.found.write_to(out)
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(AgentPortMapping {
            range: PortRange::read_from(read)?,
            found: Option::<AgentPortMappingFound>::read_from(read)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AgentPortMappingFound {
    ToAgent(AgentSessionId),
}

impl MessageEncoding for AgentPortMappingFound {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        match self {
            AgentPortMappingFound::ToAgent(id) => {
                out.write_u32::<BigEndian>(1)?;
                id.write_to(out)
            }
        }
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        match read.read_u32::<BigEndian>()? {
            1 => Ok(AgentPortMappingFound::ToAgent(AgentSessionId::read_from(read)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "unknown AgentPortMappingFound id")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UdpChannelDetails {
    pub tunnel_addr: SocketAddr,
    pub token: Arc<Vec<u8>>,
}

impl MessageEncoding for UdpChannelDetails {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        self.tunnel_addr.write_to(out)?;
        self.token.write_to(out)
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(UdpChannelDetails {
            tunnel_addr: SocketAddr::read_from(read)?,
            token: Arc::new(Vec::read_from(read)?),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Pong {
    pub request_now: u64,
    pub server_now: u64,
    pub server_id: u64,
    pub data_center_id: u32,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
    pub session_expire_at: Option<u64>,
}

impl MessageEncoding for Pong {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        out.write_u64::<BigEndian>(self.request_now)?;
        out.write_u64::<BigEndian>(self.server_now)?;
        out.write_u64::<BigEndian>(self.server_id)?;
        out.write_u32::<BigEndian>(self.data_center_id)?;
        self.client_addr.write_to(out)?;
        self.tunnel_addr.write_to(out)?;
        self.session_expire_at.write_to(out)?;

        Ok(())
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(Pong {
            request_now: read.read_u64::<BigEndian>()?,
            server_now: read.read_u64::<BigEndian>()?,
            server_id: read.read_u64::<BigEndian>()?,
            data_center_id: read.read_u32::<BigEndian>()?,
            client_addr: SocketAddr::read_from(read)?,
            tunnel_addr: SocketAddr::read_from(read)?,
            session_expire_at: Option::read_from(read)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AgentRegistered {
    pub id: AgentSessionId,
    pub expires_at: u64,
}

impl MessageEncoding for AgentRegistered {
    fn write_to<T: Write>(&self, out: &mut T) -> std::io::Result<()> {
        self.id.write_to(out)?;
        out.write_u64::<BigEndian>(self.expires_at)
    }

    fn read_from<T: Read>(read: &mut T) -> std::io::Result<Self> {
        Ok(AgentRegistered {
            id: AgentSessionId::read_from(read)?,
            expires_at: read.read_u64::<BigEndian>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::net::Ipv4Addr;

    use rand::{Rng, RngCore, thread_rng};

    use crate::PortProto;
    use crate::rpc::ControlRpcMessage;

    use super::*;

    #[test]
    fn fuzzy_test_control_request() {
        let mut rng = thread_rng();
        let mut buffer = vec![0u8; 2048];

        for _ in 0..100000 {
            let msg = rng_control_request(&mut rng);
            test_encoding(msg, &mut buffer);
        }

        for _ in 0..1000 {
            test_encoding(ControlRpcMessage {
                request_id: rng.next_u64(),
                content: rng_control_request(&mut rng),
            }, &mut buffer);
        }
    }

    #[test]
    fn fuzzy_test_control_response() {
        let mut rng = thread_rng();
        let mut buffer = vec![0u8; 2048];

        for _ in 0..100000 {
            let msg = rng_control_response(&mut rng);
            test_encoding(msg, &mut buffer);
        }

        for _ in 0..1000 {
            test_encoding(ControlRpcMessage {
                request_id: rng.next_u64(),
                content: rng_control_response(&mut rng),
            }, &mut buffer);
        }
    }

    fn test_encoding<T: MessageEncoding + PartialEq + Debug>(msg: T, buffer: &mut [u8]) {
        let mut writer = &mut buffer[..];
        msg.write_to(&mut writer).unwrap();

        let remaining_len = writer.len();
        let written = buffer.len() - remaining_len;
        let mut reader = &buffer[0..written];
        let recovered = T::read_from(&mut reader).unwrap();

        assert_eq!(msg, recovered);
    }

    pub fn rng_control_request<R: RngCore>(rng: &mut R) -> ControlRequest {
        match rng.next_u32() % 5 {
            0 => ControlRequest::Ping(Ping {
                now: rng.next_u64(),
                session_id: if rng.next_u32() % 2 == 0 {
                    Some(AgentSessionId {
                        session_id: rng.next_u64(),
                        account_id: rng.next_u64(),
                        agent_id: rng.next_u64(),
                    })
                } else {
                    None
                }
            }),
            1 => ControlRequest::AgentRegister(AgentRegister {
                account_id: rng.next_u64(),
                agent_id: rng.next_u64(),
                agent_version: rng.next_u64(),
                timestamp: rng.next_u64(),
                client_addr: rng_socket_address(rng),
                tunnel_addr: rng_socket_address(rng),
                signature: {
                    let mut bytes = [0u8; 32];
                    rng.fill(&mut bytes);
                    bytes
                },
            }),
            2 => ControlRequest::AgentKeepAlive(AgentSessionId {
                session_id: rng.next_u64(),
                account_id: rng.next_u64(),
                agent_id: rng.next_u64(),
            }),
            3 => ControlRequest::SetupUdpChannel(AgentSessionId {
                session_id: rng.next_u64(),
                account_id: rng.next_u64(),
                agent_id: rng.next_u64(),
            }),
            4 => ControlRequest::AgentCheckPortMapping(AgentCheckPortMapping {
                agent_session_id: AgentSessionId {
                    session_id: rng.next_u64(),
                    account_id: rng.next_u64(),
                    agent_id: rng.next_u64(),
                },
                port_range: PortRange {
                    ip: match rng.next_u32() % 2 {
                        0 => IpAddr::V4(Ipv4Addr::from(rng.next_u32())),
                        1 => IpAddr::V6({
                            let mut bytes = [0u8; 16];
                            rng.fill(&mut bytes);
                            bytes.into()
                        }),
                        _ => unreachable!(),
                    },
                    port_start: rng.next_u32() as u16,
                    port_end: rng.next_u32() as u16,
                    port_proto: match rng.next_u32() % 3 {
                        0 => PortProto::Tcp,
                        1 => PortProto::Udp,
                        2 => PortProto::Both,
                        _ => unreachable!(),
                    },
                },
            }),
            _ => unreachable!(),
        }
    }

    pub fn rng_control_response<R: RngCore>(rng: &mut R) -> ControlResponse {
        match rng.next_u32() % 8 {
            0 => ControlResponse::Pong(Pong {
                request_now: rng.next_u64(),
                server_now: rng.next_u64(),
                server_id: rng.next_u64(),
                data_center_id: rng.next_u32(),
                client_addr: rng_socket_address(rng),
                tunnel_addr: rng_socket_address(rng),
                session_expire_at: if rng.next_u32() % 2 == 1 {
                    Some(rng.next_u64())
                } else {
                    None
                },
            }),
            1 => ControlResponse::InvalidSignature,
            2 => ControlResponse::Unauthorized,
            3 => ControlResponse::RequestQueued,
            4 => ControlResponse::TryAgainLater,
            5 => ControlResponse::AgentRegistered(AgentRegistered {
                id: AgentSessionId {
                    session_id: rng.next_u64(),
                    account_id: rng.next_u64(),
                    agent_id: rng.next_u64(),
                },
                expires_at: rng.next_u64(),
            }),
            6 => ControlResponse::AgentPortMapping(AgentPortMapping {
                range: PortRange {
                    ip: match rng.next_u32() % 2 {
                        0 => IpAddr::V4(Ipv4Addr::from(rng.next_u32())),
                        1 => IpAddr::V6({
                            let mut bytes = [0u8; 16];
                            rng.fill(&mut bytes);
                            bytes.into()
                        }),
                        _ => unreachable!(),
                    },
                    port_start: rng.next_u32() as u16,
                    port_end: rng.next_u32() as u16,
                    port_proto: match rng.next_u32() % 3 {
                        0 => PortProto::Tcp,
                        1 => PortProto::Udp,
                        2 => PortProto::Both,
                        _ => unreachable!(),
                    },
                },
                found: match rng.next_u32() % 2 {
                    0 => None,
                    1 => Some(AgentPortMappingFound::ToAgent(AgentSessionId {
                        session_id: rng.next_u64(),
                        account_id: rng.next_u64(),
                        agent_id: rng.next_u64(),
                    })),
                    _ => unreachable!()
                },
            }),
            7 => ControlResponse::UdpChannelDetails(UdpChannelDetails {
                tunnel_addr: rng_socket_address(rng),
                token: {
                    let mut len = ((rng.next_u64() % 30) + 32) as usize;
                    let mut buffer = vec![0u8; len];
                    rng.fill_bytes(&mut buffer);
                    buffer
                },
            }),
            _ => unreachable!()
        }
    }

    fn rng_socket_address<R: RngCore>(rng: &mut R) -> SocketAddr {
        SocketAddr::new(
            match rng.next_u32() % 2 {
                0 => IpAddr::V4(Ipv4Addr::from(rng.next_u32())),
                1 => IpAddr::V6({
                    let mut bytes = [0u8; 16];
                    rng.fill(&mut bytes);
                    bytes.into()
                }),
                _ => unreachable!(),
            },
            rng.next_u32() as u16,
        )
    }
}