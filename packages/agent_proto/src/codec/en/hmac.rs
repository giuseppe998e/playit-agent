use std::io;

use bytes::BufMut;
use sha2::Sha256;

use super::Encode;
use crate::control::HmacSign;

impl Encode for HmacSign<Sha256> {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        super::ensure!(buf.remaining_mut() >= Self::SIZE);
        let bytes = self.as_slice();
        buf.put_slice(bytes);
        Ok(())
    }
}
