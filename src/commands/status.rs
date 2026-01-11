use anyhow::Result;
use colored::Colorize;
use tabled::{Table, Tabled};

use crate::digitalocean::DoClient;

#[derive(Tabled)]
struct DropletRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Public IP")]
    public_ip: String,
}

pub async fn run() -> Result<()> {
    let client = DoClient::from_config()?;
    let droplets = client.list_droplets().await?;

    let rows: Vec<DropletRow> = droplets
        .iter()
        .map(|d| {
            let status = match d.status.as_str() {
                "active" => "active".green().to_string(),
                "off" => "off".red().to_string(),
                s => s.yellow().to_string(),
            };
            DropletRow {
                name: d.name.clone(),
                status,
                public_ip: d.public_ip().unwrap_or("-").to_string(),
            }
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("{}", table);

    Ok(())
}

pub async fn list() -> Result<()> {
    let config = crate::config::Config::load()?;

    if config.droplets.is_empty() {
        println!("No droplets configured. Run `tdops init` first.");
        return Ok(());
    }

    println!("Configured droplets:\n");
    for (name, droplet) in &config.droplets {
        println!(
            "  {} (ID: {}, IP: {})",
            name.bold(),
            droplet.id,
            droplet.public_ip
        );
    }

    Ok(())
}
