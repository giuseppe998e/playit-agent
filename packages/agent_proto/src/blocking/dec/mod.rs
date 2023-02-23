mod agent;
mod control;
mod hmac;
mod socket;

use std::{
    io::{Error, ErrorKind, Read, Result},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use byteorder::{BigEndian, ReadBytesExt};

pub trait MessageDecode: Sized {
    fn read_from<R: ::std::io::Read>(input: &mut R) -> ::std::io::Result<Self>;
}

impl MessageDecode for u64 {
    #[inline]
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        input.read_u64::<BigEndian>()
    }
}

impl<T: MessageDecode> MessageDecode for Option<T> {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        match input.read_u8()? {
            0 => Ok(None),
            1 => T::read_from(input).map(Some),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not an \"Option<T>\"."),
            )),
        }
    }
}

impl MessageDecode for Vec<u8> {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let capacity = input.read_u64::<BigEndian>()? as usize;
        let mut vec = vec![0u8; capacity];

        input.read_exact(&mut vec)?;
        Ok(vec)
    }
}

impl<T: MessageDecode> MessageDecode for Vec<T> {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let capacity = input.read_u64::<BigEndian>()? as usize;
        let mut vec = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            let entry = T::read_from(input)?;
            vec.push(entry);
        }

        Ok(vec)
    }
}

impl MessageDecode for SocketAddr {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        match input.read_u8()? {
            4 => Ok(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::read_from(input)?,
                input.read_u16::<BigEndian>()?,
            ))),
            6 => Ok(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::read_from(input)?,
                input.read_u16::<BigEndian>()?,
                0,
                0,
            ))),

            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not a \"SocketAddr\"."),
            )),
        }
    }
}

impl MessageDecode for IpAddr {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        match input.read_u8()? {
            4 => Ipv4Addr::read_from(input).map(IpAddr::V4),
            6 => Ipv6Addr::read_from(input).map(IpAddr::V6),
            v => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Given input(\"{v}\") is not a \"IpAddr\"."),
            )),
        }
    }
}

impl MessageDecode for Ipv4Addr {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 4];

        input.read_exact(&mut bytes)?;
        Ok(Self::from(bytes))
    }
}

impl MessageDecode for Ipv6Addr {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 16];

        input.read_exact(&mut bytes)?;
        Ok(Self::from(bytes))
    }
}
