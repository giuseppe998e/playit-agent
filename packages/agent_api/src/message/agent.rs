use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::Authorized, ApiClient, Result};

const AGENT_ENDPOINT: &str = "/agent";

impl ApiClient<Authorized> {
    pub async fn get_control_address(&self) -> Result<ControlAddress> {
        let payload = json!({ "type": "get-control-address" });
        self.post(AGENT_ENDPOINT, &payload).await
    }

    pub async fn get_agent_status<S: Into<String>>(
        &self,
        client_version: S,
    ) -> Result<AgentAccountStatus> {
        let payload = json!({
            "type": "get-agent-account-status",
            "client_version": client_version.into()
        });

        self.post(AGENT_ENDPOINT, &payload).await
    }

    pub async fn get_secret<S: Into<String>>(&self, claim: S) -> Result<()> {
        let payload = json!({
            "type": "exchange-claim-for-secret",
            "claim_key": claim.into()
        });

        self.post(AGENT_ENDPOINT, &payload).await
    }

    // XXX what does it means?
    pub async fn sign_agent_register(
        &self,
        payload: SignAgentRegisterPayload,
    ) -> Result<SignedAgentRegister> {
        let payload = {
            let mut payload = json!(payload);

            let payload_map = payload.as_object_mut().unwrap(); // Never panics
            payload_map.insert(String::from("type"), json!("sign-agent-register"));

            payload
        };

        self.post(AGENT_ENDPOINT, &payload).await
    }
}

#[derive(Debug, Deserialize)]
pub struct ControlAddress {
    pub control_address: SocketAddr,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum AgentAccountStatus {
    #[serde(rename = "no-account")]
    NoAccount {},
    #[serde(rename = "guest-account")]
    GuestAccount {
        account_id: u64,
        web_session_key: String,
    },
    #[serde(rename = "unverified-account")]
    UnverifiedAccount { account_id: u64 },
    #[serde(rename = "verified-account")]
    VerifiedAccount { account_id: u64 },
    #[serde(rename = "user-notice")]
    UserNotice {
        message: String,
        notice_url: String,
        important: bool,
        prevent_usage: bool,
    },
}

#[derive(Debug, Deserialize)]
pub struct AgentSecret {
    pub secret_key: String,
}

#[derive(Serialize)]
pub struct SignAgentRegisterPayload {
    pub agent_version: u64,
    pub client_addr: SocketAddr,
    pub tunnel_addr: SocketAddr,
}

#[derive(Debug, Deserialize)]
pub struct SignedAgentRegister {
    pub data: String,
}
