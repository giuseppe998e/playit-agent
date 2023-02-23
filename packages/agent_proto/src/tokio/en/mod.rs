mod agent;
mod control;
mod generics;
mod hmac;
mod socket;

#[async_trait::async_trait]
pub trait AsyncMessageEncode: Sized {
    async fn write_into<W>(self, buf: &mut W) -> ::std::io::Result<()>
    where
        W: ::tokio::io::AsyncWriteExt + Unpin + Send;
}
