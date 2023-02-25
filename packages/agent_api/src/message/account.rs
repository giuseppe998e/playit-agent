use std::net::IpAddr;

use playit_agent_proto::socket::Protocol;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{client::Authorized, PlayItClient, Result};

const ACCOUNT_ENDPOINT: &str = "/account";

impl PlayItClient<Authorized> {
    pub async fn create_tunnel(&self, payload: CreateTunnelPayload) -> Result<Uuid> {
        let payload = {
            let mut payload = json!(payload);

            let payload_map = payload.as_object_mut().unwrap(); // Never panics
            payload_map.insert(String::from("type"), json!("create-tunnel"));

            payload
        };

        let response = self
            .post::<CreateTunnelResponse>(ACCOUNT_ENDPOINT, &payload)
            .await?;

        Ok(response.id)
    }

    pub async fn list_tunnels(&self) -> Result<TunnelList> {
        let payload = json!({ "type": "list-account-tunnels" });
        self.post(ACCOUNT_ENDPOINT, &payload).await
    }
}

#[derive(Deserialize, Debug)]
struct CreateTunnelResponse {
    id: Uuid,
}

#[derive(Serialize)]
pub struct CreateTunnelPayload {
    pub tunnel_type: Option<TunnelType>,
    pub name: Option<String>,
    pub port_type: Protocol,
    pub port_count: u16,
    pub local_ip: IpAddr,
    pub local_port: Option<u16>,
    pub agent_id: Option<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct TunnelList {
    pub tunnels: Vec<AccountTunnel>,
    pub agent_id: Option<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct AccountTunnel {
    pub id: Uuid,
    pub enabled: bool,
    pub name: Option<String>,
    pub ip_address: IpAddr,
    pub ip_hostname: String,
    pub custom_domain: Option<CustomDomain>,
    pub assigned_domain: String,
    pub display_address: String,
    pub is_dedicated_ip: bool,
    pub from_port: u16,
    pub to_port: u16,
    pub tunnel_type: Option<TunnelType>,
    pub port_type: Protocol,
    pub firewall_id: Option<Uuid>,
    pub protocol: TunnelProtocol,
}

#[derive(Deserialize, Debug)]
pub struct CustomDomain {
    pub id: Uuid,
    pub name: String,
    pub target: Option<CustomDomainTarget>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CustomDomainTarget {
    #[serde(rename = "port-allocation")]
    PortAllocation { id: Uuid },
    #[serde(rename = "ip-address")]
    IpAddress { ip: IpAddr },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "protocol")]
pub enum TunnelProtocol {
    #[serde(rename = "to-agent")]
    ToAgent {
        local_ip: IpAddr,
        local_port: u16,
        agent_id: Option<u64>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TunnelType {
    #[serde(rename = "minecraft-java")]
    MinecraftJava,
    #[serde(rename = "minecraft-bedrock")]
    MinecraftBedrock,
    #[serde(rename = "valheim")]
    Valheim,
    #[serde(rename = "terraria")]
    Terraria,
    #[serde(rename = "starbound")]
    Starbound,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "7days")]
    SevenDays,
    #[serde(rename = "unturned")]
    Unturned,
}
