use core::mem;
use std::{
    io::{self, Error, ErrorKind},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use bytes::Buf;

pub trait Decode: Sized {
    /// This method checks the contents of the buffer.
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()>;

    /// Using this method without having called
    /// `Decode::check(..)` could result in a `panci!(...)`.
    fn decode<B: Buf>(buf: &mut B) -> Self;
}

// Primitives
macro_rules! decode_impl {
    ( $( $type:ty, $buf_get:tt );+ $(;)? ) => {
        $(
            impl Decode for $type {
                fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
                    super::checked_advance!(buf.remaining() >= mem::size_of::<$type>());
                    Ok(())
                }

                #[inline(always)]
                fn decode<B: Buf>(buf: &mut B) -> Self {
                    buf.$buf_get()
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
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        super::checked_advance!(buf.remaining() >= LEN);
        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let mut bytes = [0u8; LEN];
        buf.copy_to_slice(&mut bytes);
        bytes
    }
}

// Option
impl<T: Decode> Decode for Option<T> {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        super::ensure!(buf.remaining() >= mem::size_of::<u8>());
        let discriminant = <u8>::decode(buf);

        match discriminant {
            0 => Ok(()),
            1 => T::check(buf),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'Option'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u8>::decode(buf);
        match discriminant {
            0 => None,
            1 => Some(T::decode(buf)),

            _ => panic!("unknown discriminant for 'Option'"),
        }
    }
}

// SocketAddr
impl Decode for SocketAddr {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        super::ensure!(buf.remaining() > mem::size_of::<u8>());
        let discriminant = <u8>::decode(buf);

        match discriminant {
            4 => SocketAddrV4::check(buf),
            6 => SocketAddrV6::check(buf),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'SocketAddr'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u8>::decode(buf);
        match discriminant {
            4 => Self::V4(SocketAddrV4::decode(buf)),
            6 => Self::V6(SocketAddrV6::decode(buf)),

            _ => panic!("unknown discriminant for 'SocketAddr'"),
        }
    }
}

impl Decode for SocketAddrV4 {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        Ipv4Addr::check(buf)?;
        <u16>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let ip = Ipv4Addr::decode(buf);
        let port = <u16>::decode(buf);

        SocketAddrV4::new(ip, port)
    }
}

impl Decode for SocketAddrV6 {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        Ipv6Addr::check(buf)?;
        <u16>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let ip = Ipv6Addr::decode(buf);
        let port = <u16>::decode(buf);

        SocketAddrV6::new(ip, port, 0, 0)
    }
}

// IpAddr
impl Decode for IpAddr {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        super::ensure!(buf.remaining() > mem::size_of::<u8>());
        let discriminant = <u8>::decode(buf);

        match discriminant {
            4 => Ipv4Addr::check(buf),
            6 => Ipv6Addr::check(buf),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'IpAddr'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u8>::decode(buf);
        match discriminant {
            4 => Self::V4(Ipv4Addr::decode(buf)),
            6 => Self::V6(Ipv6Addr::decode(buf)),

            _ => panic!("unknown discriminant for 'IpAddr'"),
        }
    }
}

impl Decode for Ipv4Addr {
    #[inline]
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        <[u8; 4]>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let bytes = <[u8; 4]>::decode(buf);
        Self::from(bytes)
    }
}

impl Decode for Ipv6Addr {
    #[inline]
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        <[u8; 16]>::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let bytes = <[u8; 16]>::decode(buf);
        Self::from(bytes)
    }
}
