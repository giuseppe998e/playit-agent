pub mod common;
pub mod request;
pub mod response;

use core::mem;
use std::io;

use bytes::{Buf, BufMut};

use crate::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoteProcedureCall<T> {
    request_id: u64,
    content: T,
}

impl<T> RemoteProcedureCall<T> {
    pub fn new(request_id: u64, content: T) -> Self {
        Self {
            request_id,
            content,
        }
    }

    pub fn request_id(&self) -> u64 {
        self.request_id
    }

    pub fn get_content(&self) -> &T {
        &self.content
    }

    pub fn unwrap(self) -> (u64, T) {
        (self.request_id, self.content)
    }
}

impl<T: Encode> Encode for RemoteProcedureCall<T> {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        self.request_id.encode(buf)?;
        self.content.encode(buf)
    }
}

impl<T: Decode> Decode for RemoteProcedureCall<T> {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() > mem::size_of::<u64>());
        T::check(buf)
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let request_id = <u64>::decode(buf);
        let content = T::decode(buf);

        Self {
            request_id,
            content,
        }
    }
}
