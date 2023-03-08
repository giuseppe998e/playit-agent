use core::mem;
use std::{
    io::{self, Error, ErrorKind},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use bytes::Buf;

pub trait Decode: Sized {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self>;
}

// Primitives
macro_rules! decode_impl {
    ( $( $type:ty, $buf_get:tt );+ $(;)? ) => {
        $(
            impl Decode for $type {
                fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
                    super::ensure!(buf.remaining() >= mem::size_of::<$type>());
                    let value = buf.$buf_get();
                    Ok(value)
                }
            }
        )+
    };
}

decode_impl!(
    u8, get_u8;
    u16, get_u16;
    u32, get_u32;
    u64, get_u64;
    u128, get_u128;

    i8, get_i8;
    i16, get_i16;
    i32, get_i32;
    i64, get_i64;
    i128, get_i128;
);

// Array of u8
impl<const LEN: usize> Decode for [u8; LEN] {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() >= mem::size_of::<Self>());

        let mut bytes = [0u8; LEN];
        buf.copy_to_slice(&mut bytes);
        Ok(bytes)
    }
}

// Option
impl<T: Decode> Decode for Option<T> {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let discriminant = <u8>::decode(buf)?;
        match discriminant {
            0 => Ok(None),
            1 => T::decode(buf).map(Some),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'Option'",
            )),
        }
    }
}

// SocketAddr
impl Decode for SocketAddr {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() > mem::size_of::<u8>());
        let discriminant = buf.get_u8();

        match discriminant {
            4 => SocketAddrV4::decode(buf).map(Self::V4),
            6 => SocketAddrV6::decode(buf).map(Self::V6),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'SocketAddr'",
            )),
        }
    }
}

impl Decode for SocketAddrV4 {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let ip = Ipv4Addr::decode(buf)?;
        let port = <u16>::decode(buf)?;

        Ok(SocketAddrV4::new(ip, port))
    }
}

impl Decode for SocketAddrV6 {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let ip = Ipv6Addr::decode(buf)?;
        let port = <u16>::decode(buf)?;

        Ok(SocketAddrV6::new(ip, port, 0, 0))
    }
}

// IpAddr
impl Decode for IpAddr {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        super::ensure!(buf.remaining() > mem::size_of::<u8>());
        let discriminant = buf.get_u8();

        match discriminant {
            4 => Ipv4Addr::decode(buf).map(Self::V4),
            6 => Ipv6Addr::decode(buf).map(Self::V6),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'IpAddr'",
            )),
        }
    }
}

impl Decode for Ipv4Addr {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let bytes = <[u8; 4]>::decode(buf)?;
        Ok(Self::from(bytes))
    }
}

impl Decode for Ipv6Addr {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        let bytes = <[u8; 16]>::decode(buf)?;
        Ok(Self::from(bytes))
    }
}
