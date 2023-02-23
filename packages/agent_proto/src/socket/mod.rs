use std::net::IpAddr;

use serde::{Deserialize, Serialize};

mod port;
pub use port::Port;

mod proto;
pub use proto::Protocol;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Socket {
    pub ip: IpAddr,
    #[serde(flatten)]
    pub port: Port,
    #[serde(rename = "port_proto")]
    pub proto: Protocol,
}

//impl From<Socket> for Vec<SocketAddr> {}
