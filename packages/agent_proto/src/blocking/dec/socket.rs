use std::{
    io::{Cursor, Error, ErrorKind, Read, Result},
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
};

use byteorder::{BigEndian, ReadBytesExt};

use crate::socket::{
    Port, Protocol, Socket, SocketFlow, SocketFlowV4, SocketFlowV6, FLOW_ID_BYTES,
    FLOW_V4_BYTES, FLOW_V4_ID, FLOW_V4_ID_OLD, FLOW_V6_BYTES, FLOW_V6_ID,
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
        let mut v4_buf = [0u8; FLOW_V4_BYTES + FLOW_ID_BYTES];
        input.read_exact(&mut v4_buf)?;

        let footer_id = {
            let mut footer_id_buf = Cursor::new(&v4_buf[FLOW_V4_BYTES..]);
            footer_id_buf.read_u64::<BigEndian>()?
        };

        if matches!(footer_id, FLOW_V4_ID | FLOW_V4_ID_OLD) {
            let mut v4_cursor = Cursor::new(&v4_buf);
            return SocketFlowV4::read_from(&mut v4_cursor).map(Self::V4);
        }

        // V6
        let mut v6_buf = [0u8; FLOW_V6_BYTES - FLOW_V4_BYTES];
        input.read_exact(&mut v6_buf)?;

        let footer_id = {
            let mut footer_id_buf =
                Cursor::new(&v6_buf[FLOW_V6_BYTES - FLOW_V4_BYTES - FLOW_ID_BYTES..]);
            footer_id_buf.read_u64::<BigEndian>()?
        };

        if matches!(footer_id, FLOW_V6_ID) {
            let mut v6_cursor = Cursor::new(&v4_buf).chain(Cursor::new(&v6_buf));
            return SocketFlowV6::read_from(&mut v6_cursor).map(Self::V6);
        }

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
