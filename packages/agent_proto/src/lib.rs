pub mod codec;
pub mod control;
pub mod socket;

use control::{
    KeepAliveRequest, Ping, Pong, PortMappingRequest, PortMappingResponse, RegisterRequest,
    RegisterResponse, UdpChannelDetails, UdpChannelRequest,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoteProcedureCall<T> {
    pub call_id: u64,
    pub content: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcRequest {
    Ping(Ping),

    KeepAlive(KeepAliveRequest),

    Register(RegisterRequest),
    UdpChannel(UdpChannelRequest),
    PortMapping(PortMappingRequest),
}

impl RpcRequest {
    pub const PING_IDX: u8 = 1;

    pub const KEEP_ALIVE_IDX: u8 = 3;

    pub const REGISTER_IDX: u8 = 2;
    pub const UPD_CHANNEL_IDX: u8 = 4;
    pub const PORT_MAPPING_IDX: u8 = 5;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::Ping(_) => Self::PING_IDX,

            Self::KeepAlive(_) => Self::KEEP_ALIVE_IDX,

            Self::Register(_) => Self::REGISTER_IDX,
            Self::UdpChannel(_) => Self::UPD_CHANNEL_IDX,
            Self::PortMapping(_) => Self::PORT_MAPPING_IDX,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcResponse {
    Pong(Pong),

    InvalidSignature,
    Unauthorized,
    RequestQueued,
    TryAgainLater,

    Register(RegisterResponse),
    UdpChannel(UdpChannelDetails),
    PortMapping(PortMappingResponse),
}

impl RpcResponse {
    pub const PONG_IDX: u8 = 1;

    pub const INVALID_SIGNATURE_IDX: u8 = 2;
    pub const UNAUTHORIZED_IDX: u8 = 3;
    pub const REQUEST_QUEUED_IDX: u8 = 4;
    pub const TRY_AGAIN_LATER_IDX: u8 = 5;

    pub const REGISTER_IDX: u8 = 6;
    pub const UPD_CHANNEL_IDX: u8 = 7;
    pub const PORT_MAPPING_IDX: u8 = 8;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::Pong(_) => Self::PONG_IDX,

            Self::InvalidSignature => Self::INVALID_SIGNATURE_IDX,
            Self::Unauthorized => Self::UNAUTHORIZED_IDX,
            Self::RequestQueued => Self::REQUEST_QUEUED_IDX,
            Self::TryAgainLater => Self::TRY_AGAIN_LATER_IDX,

            Self::Register(_) => Self::REGISTER_IDX,
            Self::UdpChannel(_) => Self::UPD_CHANNEL_IDX,
            Self::PortMapping(_) => Self::PORT_MAPPING_IDX,
        }
    }
}
