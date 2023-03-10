mod client;

use core::mem;
use std::io::{self, Error, ErrorKind};

use bytes::{Buf, BufMut};

use crate::{
    rpc::{response::RpcResponse, RemoteProcedureCall},
    Decode, Encode,
};

pub use self::client::{ClaimInstructions, ClientDetails};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlFeed {
    RpcResponse(RemoteProcedureCall<RpcResponse>),
    NewClient(ClientDetails),
}

impl ControlFeed {
    pub const RPC_RESP_IDX: u8 = 1;
    pub const NEW_CLIENT_IDX: u8 = 2;

    pub fn discrimintant(&self) -> u8 {
        match self {
            Self::RpcResponse(_) => Self::RPC_RESP_IDX,
            Self::NewClient(_) => Self::NEW_CLIENT_IDX,
        }
    }
}

impl Encode for ControlFeed {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() > mem::size_of::<u32>());
        match self {
            ControlFeed::RpcResponse(rpc) => {
                buf.put_u32(1);
                rpc.encode(buf)
            }
            ControlFeed::NewClient(client) => {
                buf.put_u32(2);
                client.encode(buf)
            }
        }
    }
}

impl Decode for ControlFeed {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<B>) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining() >= mem::size_of::<u32>());
        let discriminant = <u32>::decode(buf) as u8;

        match discriminant {
            Self::RPC_RESP_IDX => RemoteProcedureCall::<RpcResponse>::check(buf),
            Self::NEW_CLIENT_IDX => ClientDetails::check(buf),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "unknown discriminant for 'ControlFeed'",
            )),
        }
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let discriminant = <u32>::decode(buf) as u8;
        match discriminant {
            Self::RPC_RESP_IDX => {
                let rpc = RemoteProcedureCall::<RpcResponse>::decode(buf);
                Self::RpcResponse(rpc)
            }
            Self::NEW_CLIENT_IDX => Self::NewClient(ClientDetails::decode(buf)),

            _ => panic!("unknown discriminant for 'ControlFeed'"),
        }
    }
}
