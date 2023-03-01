use crate::agent::AgentSession;

#[async_trait::async_trait]
impl super::AsyncMessageEncode for AgentSession {
    async fn write_into<W>(self, buf: &mut W) -> ::std::io::Result<()>
    where
        W: ::tokio::io::AsyncWriteExt + ?Sized + Unpin + Send,
    {
        buf.write_u64(self.id).await?;
        buf.write_u64(self.account_id).await?;
        buf.write_u64(self.agent_id).await
    }
}
