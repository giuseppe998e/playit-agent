use core::mem;
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
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining() >= mem::size_of::<u8>());
        let discriminant = <u8>::decode(buf);

        match discriminant {
            1..=3 => Ok(()),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'Protocol'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u8>::decode(buf);
        match discriminant {
            1 => Self::Tcp,
            2 => Self::Udp,
            3 => Self::Both,

            _ => panic!("unknown discriminant for 'Protocol'"),
        }
    }
}
