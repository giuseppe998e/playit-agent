mod flow;
mod port;
mod proto;

use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};

pub use flow::{SocketFlow, SocketFlowV4, SocketFlowV6};
pub(super) use flow::{V4_FOOTER_ID, V4_FOOTER_ID_OLD, V4_LEN, V6_FOOTER_ID, V6_LEN};
pub use port::{Port, PortRange};
pub use proto::Protocol;

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

// TODO Proper error needed
// impl TryFrom<Vec<SocketAddr>> for Socket {
//     type Error = ();
//
//     fn try_from(value: Vec<SocketAddr>) -> Result<Self, Self::Error> {
//         if value.len() == 0 {
//             return Err(());
//         }
//
//         let (mut max, mut min) = (u16::MIN, u16::MAX);
//         let addr = {
//             let addr = value.iter().next();
//             value.iter().map(Some).fold(addr, |init, addr| {
//                 let (init_val, addr_val) = (init.unwrap(), addr.unwrap());
//                 let addr_port = addr_val.port();
//
//                 if addr_port < min {
//                     min = addr_port;
//                 }
//
//                 if addr_port > max {
//                     max = addr_port;
//                 }
//
//                 (init_val.ip() == addr_val.ip()).then_some(init_val)
//             })
//         };
//
//         match addr {
//             Some(addr_val) => {
//                 let ip = addr_val.ip();
//                 let port = Port::new(min, Some(max));
//
//                 Ok(Self {
//                     ip,
//                     port,
//                     proto: Protocol::Both,
//                 })
//             }
//             None => Err(()),
//         }
//     }
// }
