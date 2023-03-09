mod port;
mod proto;

use std::{
    io,
    net::{IpAddr, SocketAddr},
};

use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};

pub use port::{Port, PortRange};
pub use proto::Protocol;

use crate::codec::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Socket {
    pub ip: IpAddr,
    #[serde(flatten)]
    pub port: Port,
    #[serde(rename = "port_proto")]
    pub proto: Protocol,
}

impl From<Socket> for Vec<SocketAddr> {
    fn from(value: Socket) -> Self {
        value
            .port
            .into_iter()
            .map(|port| SocketAddr::new(value.ip, port))
            .collect()
    }
}

impl Encode for Socket {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.ip.encode(buf)?;
        self.port.encode(buf)?;
        self.proto.encode(buf)
    }
}

impl Decode for Socket {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        IpAddr::check(buf)?;
        Port::check(buf)?;
        Protocol::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let ip = IpAddr::decode(buf);
        let port = Port::decode(buf);
        let proto = Protocol::decode(buf);

        Self { ip, port, proto }
    }
}
