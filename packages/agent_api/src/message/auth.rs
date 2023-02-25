use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{client::Authorized, PlayItClient, Result};

const AUTH_ENDPOINT: &str = "/login";

impl PlayItClient<Authorized> {
    pub async fn create_guest_session(&self) -> Result<GuestSession> {
        let payload = json!({ "type": "create-guest-session" });
        self.post(AUTH_ENDPOINT, &payload).await
    }

    pub async fn get_session(&self) -> Result<SessionStatus> {
        let payload = json!({ "type": "get-session" });
        self.post(AUTH_ENDPOINT, &payload).await
    }
}

#[derive(Deserialize, Debug)]
pub struct GuestSession {
    pub account_id: u64,
    pub session_key: String,
    pub is_guest: bool,
    pub email_verified: bool,
}

#[derive(Deserialize, Debug)]
pub struct SessionStatus {
    pub account_id: u64,
    pub is_guest: bool,
    pub email_verified: bool,
    pub agent_id: Option<Uuid>,
    pub notice: Option<Notice>,
}

#[derive(Deserialize, Debug)]
pub struct Notice {
    pub url: String,
    pub message: String,
}
