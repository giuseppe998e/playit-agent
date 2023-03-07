use sha2::Sha256;

use crate::hmac::HmacSign;

// XXX Not generic on "Digest" because there is no method to distinguish them
#[async_trait::async_trait]
impl super::AsyncMessageEncode for HmacSign<Sha256> {
    async fn write_into<W>(self, buf: &mut W) -> ::std::io::Result<()>
    where
        W: ::tokio::io::AsyncWriteExt + ?Sized + Unpin + Send,
    {
        let bytes = self.as_slice();
        buf.write_all(bytes).await
    }
}
