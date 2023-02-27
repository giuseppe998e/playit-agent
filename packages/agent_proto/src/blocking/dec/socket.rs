use std::{
    io::{Cursor, Error, ErrorKind, Read, Result},
    mem,
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
};

use byteorder::{BigEndian, ReadBytesExt};

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, V4_FOOTER_ID, V4_FOOTER_ID_OLD,
    V4_LEN, V6_FOOTER_ID, V6_LEN,
};

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

// SocketFlow
impl MessageDecode for SocketFlow {
    /// To read a `SocketFlow` and determine whether it's a `SocketFlowV4` or `SocketFlowV6`,
    /// we need to look at the `footer_id` value, which is located after the structure's bytes.
    /// To deal with this inconvenience, we assume that the structure we're reading is
    /// `SocketFlowV4`, which has fewer bytes than `SocketFlowV6`, and we add the size of the
    /// `footer_id`. If the `footer_id` matches one of the expected values, we return the
    /// `SocketFlowV4` structure. Otherwise, we continue reading the remaining bytes
    /// to obtain the `SocketFlowV6` structure.
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        // Initial length of buffer to read
        const INIT_LEN: usize = V4_LEN + mem::size_of::<u64>();

        // Initialize a buffer to hold the input bytes
        let mut input_buf = Vec::<u8>::new();

        // Parse the `SocketFlowV4` variant
        // Read `SocketFlowV4` structure plus `footer_id` (20 bytes)
        input_buf.resize(INIT_LEN, 0);
        input.read_exact(&mut input_buf)?;

        // Parse `footer_id`
        let mut footer_id_bytes = &input_buf[V4_LEN..];
        let footer_id = footer_id_bytes.read_u64::<BigEndian>()?;

        // Check and parse `SocketFlowV4`
        if matches!(footer_id, V4_FOOTER_ID | V4_FOOTER_ID_OLD) {
            let mut v4_cursor = Cursor::new(&input_buf[..V4_LEN]);
            return SocketFlowV4::read_from(&mut v4_cursor).map(Self::V4);
        }

        // If `footer_id` did not match any `SocketFlowV4` variant,
        // parse the `SocketFlowV6` variant
        // Read `SocketFlowV6` structure plus `footer_id` (48 bytes)
        input_buf.resize(INIT_LEN + V6_LEN - V4_LEN, 0);
        input.read_exact(&mut input_buf[INIT_LEN..])?;

        // Parse `footer_id`
        let mut footer_id_bytes = &input_buf[V6_LEN..];
        let footer_id = footer_id_bytes.read_u64::<BigEndian>()?;

        // Check and parse `SocketFlowV6`
        if matches!(footer_id, V6_FOOTER_ID) {
            let mut v6_cursor = Cursor::new(&input_buf[..V6_LEN]);
            return SocketFlowV6::read_from(&mut v6_cursor).map(Self::V6);
        }

        // If `footer_id` did not match any `SocketFlow` variant, return an error
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid input for `SocketFlow`",
        ))
    }
}

impl MessageDecode for SocketFlowV4 {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let src_ip = input.read_u32::<BigEndian>()?;
        let dest_ip = input.read_u32::<BigEndian>()?;
        let src_port = input.read_u16::<BigEndian>()?;
        let dest_port = input.read_u16::<BigEndian>()?;

        let src = SocketAddrV4::new(src_ip.into(), src_port);
        let dest = SocketAddrV4::new(dest_ip.into(), dest_port);

        Ok(Self::new(src, dest))
    }
}

impl MessageDecode for SocketFlowV6 {
    fn read_from<R: Read>(input: &mut R) -> Result<Self> {
        let src_ip = input.read_u128::<BigEndian>()?;
        let dest_ip = input.read_u128::<BigEndian>()?;
        let src_port = input.read_u16::<BigEndian>()?;
        let dest_port = input.read_u16::<BigEndian>()?;
        let flowinfo = input.read_u32::<BigEndian>()?;

        let src = SocketAddrV6::new(src_ip.into(), src_port, flowinfo, 0);
        let dest = SocketAddrV6::new(dest_ip.into(), dest_port, flowinfo, 0);

        Ok(Self::new(src, dest, flowinfo))
    }
}
