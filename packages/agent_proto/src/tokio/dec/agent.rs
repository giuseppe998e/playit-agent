use crate::agent::AgentSession;

#[async_trait::async_trait]
impl super::AsyncMessageDecode for AgentSession {
    async fn read_from<R>(input: &mut R) -> ::std::io::Result<Self>
    where
        R: ::tokio::io::AsyncReadExt + Unpin + Send,
    {
        let session_id = input.read_u64().await?;
        let account_id = input.read_u64().await?;
        let agent_id = input.read_u64().await?;

        Ok(Self {
            session_id,
            account_id,
            agent_id,
        })
    }
}
