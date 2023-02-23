mod agent;
mod control;
mod hmac;
mod socket;

use std::{
    io::{Result, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use byteorder::{BigEndian, WriteBytesExt};

pub trait MessageEncode: Sized {
    fn write_into<W: ::std::io::Write>(self, buf: &mut W) -> ::std::io::Result<()>;
}

impl MessageEncode for u64 {
    #[inline]
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self)
    }
}

impl<T: MessageEncode> MessageEncode for Option<T> {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            Some(v) => {
                buf.write_u8(1)?;
                v.write_into(buf)
            }
            None => buf.write_u8(0),
        }
    }
}

impl MessageEncode for Vec<u8> {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.len() as _)?;
        buf.write_all(&self)
    }
}

impl<T: MessageEncode> MessageEncode for Vec<T> {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.len() as _)?;

        let mut bytes = Vec::with_capacity(self.len() * std::mem::size_of::<T>());
        self.into_iter()
            .try_for_each(|e| e.write_into(&mut bytes))?;

        buf.write_all(&bytes)
    }
}

impl MessageEncode for SocketAddr {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            SocketAddr::V4(addr) => {
                buf.write_u8(4)?;
                addr.ip().write_into(buf)?;
                buf.write_u16::<BigEndian>(addr.port())
            }
            SocketAddr::V6(addr) => {
                buf.write_u8(6)?;
                addr.ip().write_into(buf)?;
                buf.write_u16::<BigEndian>(addr.port())
            }
        }
    }
}

impl MessageEncode for IpAddr {
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        match self {
            IpAddr::V4(ip) => {
                buf.write_u8(4)?;
                ip.write_into(buf)
            }
            IpAddr::V6(ip) => {
                buf.write_u8(6)?;
                ip.write_into(buf)
            }
        }
    }
}

impl MessageEncode for Ipv4Addr {
    #[inline]
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.octets())
    }
}

impl MessageEncode for Ipv6Addr {
    #[inline]
    fn write_into<W: Write>(self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.octets())
    }
}
