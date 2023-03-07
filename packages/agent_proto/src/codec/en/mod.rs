mod control;
mod hmac;
mod socket;

use core::mem;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use bytes::{BufMut, Bytes, BytesMut};

// export to sub-modules
use super::ensure;

pub trait Encode: Sized {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()>;

    fn encode_into_bytes(self) -> io::Result<Bytes> {
        let mut bytes = BytesMut::with_capacity(mem::size_of::<Self>());
        self.encode(&mut bytes)?;
        Ok(bytes.freeze())
    }
}

// Primitives
macro_rules! encode_impl {
    ( $( $type:ty, $buf_put:tt );+ $(;)? ) => {
        $(
            impl Encode for $type {
                fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
                    super::ensure!(buf.remaining_mut() >= mem::size_of::<Self>());
                    let value = buf.$buf_put(self);
                    Ok(value)
                }
            }
        )+
    };
}

encode_impl!(
    u8, put_u8;
    u16, put_u16;
    u32, put_u32;
    u64, put_u64;
    u128, put_u128;

    i8, put_i8;
    i16, put_i16;
    i32, put_i32;
    i64, put_i64;
    i128, put_i128;
);

// Array of u8
impl<const LEN: usize> Encode for [u8; LEN] {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() >= mem::size_of::<Self>());
        buf.put_slice(&self);
        Ok(())
    }
}

// Option
impl<T: Encode> Encode for Option<T> {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() >= mem::size_of::<u8>());

        match self {
            None => buf.put_u8(0),
            Some(v) => {
                buf.put_u8(1);
                v.encode(buf)?;
            }
        }

        Ok(())
    }
}

// SocketAddress
impl Encode for SocketAddr {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u8>());

        match self {
            Self::V4(v4) => {
                buf.put_u8(4);
                v4.encode(buf)
            }
            Self::V6(v6) => {
                buf.put_u8(6);
                v6.encode(buf)
            }
        }
    }
}

impl Encode for SocketAddrV4 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        let ip = self.ip();
        ip.encode(buf)?;

        let port = self.port();
        port.encode(buf)
    }
}

impl Encode for SocketAddrV6 {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        let ip = self.ip();
        ip.encode(buf)?;

        let port = self.port();
        port.encode(buf)
    }
}

// IpAddr
impl Encode for IpAddr {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() > mem::size_of::<u8>());

        match self {
            Self::V4(v4) => {
                buf.put_u8(4);
                v4.encode(buf)
            }
            Self::V6(v6) => {
                buf.put_u8(6);
                v6.encode(buf)
            }
        }
    }
}

impl Encode for Ipv4Addr {
    #[inline]
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.octets().encode(buf)
    }
}

impl Encode for Ipv6Addr {
    #[inline]
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.octets().encode(buf)
    }
}
