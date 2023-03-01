use std::{
    mem,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};

pub const FLOW_V6_ID: u64 = 0x6668_676F_6861_6366;
pub const FLOW_V4_ID: u64 = 0x4448_474F_4841_4344;
pub const FLOW_V4_ID_OLD: u64 = 0x5CB8_67CF_7881_73B2;

pub const FLOW_ID_SIZE: usize = mem::size_of::<u64>();
const FLOW_V4_SIZE: usize = 12;
const FLOW_V6_SIZE: usize = 40;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketFlow {
    V4(SocketFlowV4),
    V6(SocketFlowV6),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SocketFlowV4 {
    src: SocketAddrV4,
    dest: SocketAddrV4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SocketFlowV6 {
    src: SocketAddrV6,
    dest: SocketAddrV6,
    // XXX Does it refer to src or dest?
    flowinfo: u32,
}

impl SocketFlow {
    pub fn src(&self) -> SocketAddr {
        match self {
            Self::V4(flow) => flow.src.into(),
            Self::V6(flow) => flow.src.into(),
        }
    }

    pub fn dest(&self) -> SocketAddr {
        match self {
            Self::V4(flow) => flow.dest.into(),
            Self::V6(flow) => flow.dest.into(),
        }
    }

    pub fn flowinfo(&self) -> Option<u32> {
        match self {
            SocketFlow::V6(flow) => Some(flow.flowinfo),
            SocketFlow::V4(_) => None,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::V4(flow) => Self::V4(flow.flip()),
            Self::V6(flow) => Self::V6(flow.flip()),
        }
    }

    pub fn size(&self) -> usize {
        let inner = match self {
            SocketFlow::V4(_) => SocketFlowV4::size(),
            SocketFlow::V6(_) => SocketFlowV6::size(),
        };

        inner + FLOW_ID_SIZE
    }

    pub fn is_ipv4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::V6(_))
    }
}

impl SocketFlowV4 {
    pub fn new(src: SocketAddrV4, dest: SocketAddrV4) -> Self {
        Self { src, dest }
    }

    pub fn src(&self) -> &SocketAddrV4 {
        &self.src
    }

    pub fn dest(&self) -> &SocketAddrV4 {
        &self.dest
    }

    pub fn flip(self) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
        }
    }

    pub const fn size() -> usize {
        FLOW_V4_SIZE
    }
}

impl From<SocketFlowV4> for SocketFlow {
    fn from(value: SocketFlowV4) -> Self {
        Self::V4(value)
    }
}

impl SocketFlowV6 {
    pub fn new(src: SocketAddrV6, dest: SocketAddrV6, flowinfo: u32) -> Self {
        Self {
            src,
            dest,
            flowinfo,
        }
    }

    pub fn src(&self) -> &SocketAddrV6 {
        &self.src
    }

    pub fn dest(&self) -> &SocketAddrV6 {
        &self.dest
    }

    pub fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    pub fn flip(self) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            flowinfo: self.flowinfo,
        }
    }

    pub const fn size() -> usize {
        FLOW_V6_SIZE
    }
}

impl From<SocketFlowV6> for SocketFlow {
    fn from(value: SocketFlowV6) -> Self {
        Self::V6(value)
    }
}
