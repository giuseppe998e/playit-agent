use std::io;

use bytes::Buf;
use hmac::digest::generic_array::GenericArray;
use sha2::Sha256;

use super::Decode;
use crate::control::HmacSign;

impl Decode for HmacSign<Sha256> {
    fn decode<B: Buf>(buf: &mut B) -> io::Result<Self> {
        const SHA256_SIZE: usize = 32;
        super::ensure!(buf.remaining() >= SHA256_SIZE);

        let bytes = buf.copy_to_bytes(SHA256_SIZE);
        let bytes = GenericArray::<u8, _>::clone_from_slice(&bytes);
        Ok(Self(bytes))
    }
}
