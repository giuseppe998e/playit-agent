use hmac::digest::{generic_array::GenericArray, OutputSizeUser};
use sha2::Sha256;

use crate::hmac::HmacSign;

// XXX Not generic on "Digest" because there is no method to distinguish them
impl super::MessageDecode for HmacSign<Sha256> {
    fn read_from<R: ::std::io::Read + ?Sized>(input: &mut R) -> ::std::io::Result<Self> {
        let mut buf = [0u8; Self::BYTES];
        input.read_exact(&mut buf)?;

        let bytes =
            GenericArray::<u8, <Sha256 as OutputSizeUser>::OutputSize>::clone_from_slice(&buf);
        Ok(Self(bytes))
    }
}
