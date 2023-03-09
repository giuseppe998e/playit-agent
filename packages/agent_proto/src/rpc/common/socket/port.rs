use core::mem;
use std::{io, iter, num::NonZeroU16, ops::RangeInclusive};

use bytes::{Buf, BufMut};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use crate::codec::{Decode, Encode};

pub type PortRange = RangeInclusive<u16>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Port {
    Single(u16),
    Range(PortRange),
}

impl Port {
    pub fn new(from: u16, to: Option<u16>) -> Self {
        match to {
            Some(to) if from < to => Self::Range(from..=to),
            _ => Self::Single(from),
        }
    }
}

impl IntoIterator for Port {
    type Item = u16;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Port::Single(port) => Box::new(iter::once(port)),
            Port::Range(range) => Box::new(range),
        }
    }
}

impl From<Port> for PortRange {
    fn from(value: Port) -> Self {
        match value {
            Port::Single(port) => port..=port,
            Port::Range(range) => range,
        }
    }
}

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Self::Single(value)
    }
}

impl From<PortRange> for Port {
    fn from(value: PortRange) -> Self {
        let (from, to) = value.into_inner();
        match (from, to) {
            _ if from < to => Self::Range(from..=to),
            _ => Self::Single(from),
        }
    }
}

impl Encode for Port {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() >= mem::size_of::<u16>() * 2);

        let (start, end) = match self {
            Self::Single(port) => (port, port),
            Self::Range(range) => (*range.start(), *range.end()),
        };

        buf.put_u16(start);
        buf.put_u16(end);

        Ok(())
    }
}

impl Decode for Port {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() >= mem::size_of::<u16>() * 2);
        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let start = <u16>::decode(buf);
        let end = <u16>::decode(buf);

        match start == end {
            true => Self::Single(start),
            false => Self::Range(start..=end),
        }
    }
}

impl Serialize for Port {
    fn serialize<S>(&self, serial: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serial.serialize_struct("port", 2)?;

        match self {
            Self::Single(port) => {
                state.serialize_field("port_start", port)?;
                state.serialize_field("port_end", port)?;
            }
            Self::Range(range) => {
                state.serialize_field("port_start", range.start())?;
                state.serialize_field("port_end", range.end())?;
            }
        }

        state.end()
    }
}

impl<'d> Deserialize<'d> for Port {
    fn deserialize<D>(deser: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        #[derive(Deserialize)]
        struct PortRaw {
            port: Option<NonZeroU16>,
            port_start: Option<NonZeroU16>,
            port_end: Option<NonZeroU16>,
        }

        let PortRaw {
            port,
            port_start,
            port_end,
        } = PortRaw::deserialize(deser)?;
        match (port, port_start, port_end) {
            (Some(value), None, None) => Ok(Self::Single(value.get())),
            (None, Some(value), None) => Ok(Self::Single(value.get())),
            (None, Some(from), Some(to)) if from == to => Ok(Self::Single(from.get())),
            (None, Some(from), Some(to)) if from < to => Ok(Self::Range(from.get()..=to.get())),
            _ => Err(serde::de::Error::custom(
                "failed to deserialize socket port(s).",
            )),
        }
    }
}
