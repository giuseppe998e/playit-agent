use std::{
    io::{Error, ErrorKind, Read, Result},
    net::IpAddr,
};

use byteorder::{BigEndian, ReadBytesExt};

use crate::socket::{Port, Protocol, Socket};

use super::MessageDecode;

impl MessageDecode for Socket {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let ip = IpAddr::read_from(input)?;
        let port = Port::read_from(input)?;
        let proto = Protocol::read_from(input)?;

        Ok(Self { ip, port, proto })
    }
}

impl MessageDecode for Port {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let start = input.read_u16::<BigEndian>()?;
        let end = input.read_u16::<BigEndian>()?;

        match start == end {
            true => Ok(Self::Single(start)),
            false => Ok(Self::Range(start..=end)),
        }
    }
}

impl MessageDecode for Protocol {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        match input.read_u8()? {
            1 => Ok(Self::Tcp),
            2 => Ok(Self::Udp),
            3 => Ok(Self::Both),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"socket::Protocol\"."),
            )),
        }
    }
}
