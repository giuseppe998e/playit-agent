mod agent;
mod control;
mod generics;
mod hmac;
mod socket;

#[async_trait::async_trait]
pub trait AsyncMessageDecode: Sized {
    async fn read_from<R>(input: &mut R) -> ::std::io::Result<Self>
    where
        R: ::tokio::io::AsyncReadExt + Unpin + Send;
}
