use std::{iter, num::NonZeroU16, ops::RangeInclusive};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

pub type PortRange = RangeInclusive<u16>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Port {
    Single(u16),
    Range(PortRange),
}

impl Port {
    pub fn into_iter(self) -> Box<dyn Iterator<Item = u16>> {
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

impl From<PortRange> for Port {
    fn from(value: PortRange) -> Self {
        let (from, to) = value.into_inner();
        match (from, to) {
            (_, _) if from < to => return Self::Range(from..=to),
            _ => return Self::Single(from),
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
