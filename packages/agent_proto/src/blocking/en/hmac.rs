use sha2::Sha256;

use crate::hmac::HmacSign;

// XXX Not generic on "Digest" because there is no method to distinguish them
impl super::MessageEncode for HmacSign<Sha256> {
    fn write_into<W: ::std::io::Write + ?Sized>(self, buf: &mut W) -> ::std::io::Result<()> {
        let bytes = self.as_slice();
        buf.write_all(bytes)
    }
}
