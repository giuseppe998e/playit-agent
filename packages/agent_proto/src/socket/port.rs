use std::{num::NonZeroU16, ops::RangeInclusive};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Port {
    Single(u16),
    Range(RangeInclusive<u16>),
}

// TODO iterator

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
            (Some(v), None, None) => Ok(Self::Single(v.get())),
            (None, Some(s), None) => Ok(Self::Single(s.get())),
            (None, Some(s), Some(e)) if s == e => Ok(Self::Single(s.get())),
            (None, Some(s), Some(e)) if s < e => Ok(Self::Range(s.get()..=e.get())),
            _ => Err(serde::de::Error::custom(
                "failed to deserialize socket port(s).",
            )),
        }
    }
}
