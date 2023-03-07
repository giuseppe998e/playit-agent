use hmac::digest::{generic_array::GenericArray, OutputSizeUser};
use sha2::Sha256;

use crate::hmac::HmacSign;

// XXX Not generic on "Digest" because there is no method to distinguish them
#[async_trait::async_trait]
impl super::AsyncMessageDecode for HmacSign<Sha256> {
    async fn read_from<R>(input: &mut R) -> ::std::io::Result<Self>
    where
        R: ::tokio::io::AsyncReadExt + ?Sized + Unpin + Send,
    {
        let mut buf = [0u8; Self::BYTES];
        input.read_exact(&mut buf).await?;

        let bytes =
            GenericArray::<u8, <Sha256 as OutputSizeUser>::OutputSize>::clone_from_slice(&buf);
        Ok(Self(bytes))
    }
}
