mod agent;
mod control;
mod hmac;
mod socket;

use std::{
    io::{self, Result, Write},
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use byteorder::{BigEndian, WriteBytesExt};
pub trait MessageEncode: Sized {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> io::Result<()>;

    fn write_into_vec(self) -> io::Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(mem::size_of::<Self>());
        Self::write_to(self, &mut vec)?;
        Ok(vec)
    }

    #[cfg(feature = "bytes")]
    #[inline]
    fn write_to_buf<B: bytes::BufMut>(self, buf: &mut B) -> io::Result<()> {
        let mut writer = bytes::BufMut::writer(buf);
        Self::write_to(self, &mut writer)
    }

    #[cfg(feature = "bytes")]
    fn write_into_bytes(self) -> io::Result<bytes::BytesMut> {
        let mut bytes = bytes::BytesMut::with_capacity(mem::size_of::<Self>());
        Self::write_to_buf(self, &mut bytes)?;
        Ok(bytes)
    }
}

impl MessageEncode for u64 {
    #[inline]
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self)
    }
}

impl<T: MessageEncode> MessageEncode for Option<T> {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        match self {
            Some(v) => {
                buf.write_u8(1)?;
                v.write_to(buf)
            }
            None => buf.write_u8(0),
        }
    }
}

impl MessageEncode for Vec<u8> {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.len() as _)?;
        buf.write_all(&self)
    }
}

impl<T: MessageEncode> MessageEncode for Vec<T> {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_u64::<BigEndian>(self.len() as _)?;

        let mut bytes = Vec::with_capacity(self.len() * std::mem::size_of::<T>());
        self.into_iter().try_for_each(|e| e.write_to(&mut bytes))?;

        buf.write_all(&bytes)
    }
}

impl MessageEncode for SocketAddr {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        match self {
            SocketAddr::V4(addr) => {
                buf.write_u8(4)?;
                addr.ip().write_to(buf)?;
                buf.write_u16::<BigEndian>(addr.port())
            }
            SocketAddr::V6(addr) => {
                buf.write_u8(6)?;
                addr.ip().write_to(buf)?;
                buf.write_u16::<BigEndian>(addr.port())
            }
        }
    }
}

impl MessageEncode for IpAddr {
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        match self {
            IpAddr::V4(ip) => {
                buf.write_u8(4)?;
                ip.write_to(buf)
            }
            IpAddr::V6(ip) => {
                buf.write_u8(6)?;
                ip.write_to(buf)
            }
        }
    }
}

impl MessageEncode for Ipv4Addr {
    #[inline]
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.octets())
    }
}

impl MessageEncode for Ipv6Addr {
    #[inline]
    fn write_to<W: Write + ?Sized>(self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.octets())
    }
}
