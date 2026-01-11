use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub do_token: Option<String>,
    #[serde(default)]
    pub droplets: HashMap<String, DropletConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropletConfig {
    pub id: u64,
    pub public_ip: String,
    pub tailscale_ip: Option<String>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tdops")
            .join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path))?;
        toml::from_str(&content).context("Failed to parse config")
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn get_droplet(&self, name: &str) -> Option<&DropletConfig> {
        let name_lower = name.to_lowercase();
        self.droplets.get(&name_lower)
    }
}

pub async fn init() -> Result<()> {
    use std::io::{self, Write};

    println!("Initializing tdops config...\n");

    // Get DO token
    print!("DigitalOcean API token: ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    // Fetch droplets from DO API
    println!("\nFetching droplets from DigitalOcean...");
    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .get("https://api.digitalocean.com/v2/droplets")
        .bearer_auth(&token)
        .send()
        .await?
        .json()
        .await?;

    let mut droplets = HashMap::new();
    if let Some(arr) = resp["droplets"].as_array() {
        for d in arr {
            let name = d["name"].as_str().unwrap_or("").to_lowercase();
            let id = d["id"].as_u64().unwrap_or(0);
            let public_ip = d["networks"]["v4"]
                .as_array()
                .and_then(|nets| {
                    nets.iter()
                        .find(|n| n["type"] == "public")
                        .and_then(|n| n["ip_address"].as_str())
                })
                .unwrap_or("")
                .to_string();

            if !name.is_empty() && id > 0 {
                droplets.insert(
                    name.clone(),
                    DropletConfig {
                        id,
                        public_ip,
                        tailscale_ip: None,
                    },
                );
                println!("  Found: {} (ID: {})", name, id);
            }
        }
    }

    let config = Config {
        do_token: Some(token),
        droplets,
    };
    config.save()?;
    println!("\nConfig saved to {:?}", Config::path());

    Ok(())
}
