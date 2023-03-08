use std::io::{self, Error, ErrorKind};

use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};

use crate::codec::{Decode, Encode};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "both")]
    Both,
}

impl From<Protocol> for u8 {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::Tcp => 1,
            Protocol::Udp => 2,
            Protocol::Both => 3,
        }
    }
}

impl Encode for Protocol {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        let byte: u8 = self.into();
        byte.encode(buf)
    }
}

impl Decode for Protocol {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u8>::decode(buf)?;
        match discriminant {
            1 => Ok(Self::Tcp),
            2 => Ok(Self::Udp),
            3 => Ok(Self::Both),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'Protocol'",
            )),
        }
    }
}
