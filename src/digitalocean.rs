use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct Droplet {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub networks: Networks,
}

#[derive(Debug, Deserialize)]
pub struct Networks {
    pub v4: Vec<NetworkV4>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkV4 {
    pub ip_address: String,
    #[serde(rename = "type")]
    pub net_type: String,
}

impl Droplet {
    pub fn public_ip(&self) -> Option<&str> {
        self.networks
            .v4
            .iter()
            .find(|n| n.net_type == "public")
            .map(|n| n.ip_address.as_str())
    }
}

#[derive(Debug, Deserialize)]
struct DropletsResponse {
    droplets: Vec<Droplet>,
}

#[derive(Debug, Deserialize)]
struct ActionResponse {
    action: Action,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    pub id: u64,
    pub status: String,
}

pub struct DoClient {
    client: Client,
    token: String,
}

impl DoClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub fn from_config() -> Result<Self> {
        let config = Config::load()?;
        let token = config
            .do_token
            .context("No DO token configured. Run `tdops init` first.")?;
        Ok(Self::new(token))
    }

    pub async fn list_droplets(&self) -> Result<Vec<Droplet>> {
        let resp: DropletsResponse = self
            .client
            .get("https://api.digitalocean.com/v2/droplets")
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.droplets)
    }

    pub async fn power_on(&self, droplet_id: u64) -> Result<Action> {
        self.droplet_action(droplet_id, "power_on").await
    }

    pub async fn power_off(&self, droplet_id: u64) -> Result<Action> {
        self.droplet_action(droplet_id, "power_off").await
    }

    pub async fn reboot(&self, droplet_id: u64) -> Result<Action> {
        self.droplet_action(droplet_id, "reboot").await
    }

    pub async fn snapshot(&self, droplet_id: u64, name: &str) -> Result<Action> {
        let resp: ActionResponse = self
            .client
            .post(format!(
                "https://api.digitalocean.com/v2/droplets/{}/actions",
                droplet_id
            ))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({
                "type": "snapshot",
                "name": name
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.action)
    }

    async fn droplet_action(&self, droplet_id: u64, action_type: &str) -> Result<Action> {
        let resp: ActionResponse = self
            .client
            .post(format!(
                "https://api.digitalocean.com/v2/droplets/{}/actions",
                droplet_id
            ))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "type": action_type }))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.action)
    }

    pub async fn wait_for_action(&self, droplet_id: u64, action_id: u64) -> Result<()> {
        loop {
            let resp: ActionResponse = self
                .client
                .get(format!(
                    "https://api.digitalocean.com/v2/droplets/{}/actions/{}",
                    droplet_id, action_id
                ))
                .bearer_auth(&self.token)
                .send()
                .await?
                .json()
                .await?;

            match resp.action.status.as_str() {
                "completed" => return Ok(()),
                "errored" => anyhow::bail!("Action failed"),
                _ => tokio::time::sleep(tokio::time::Duration::from_secs(2)).await,
            }
        }
    }
}
