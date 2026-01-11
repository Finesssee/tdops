use anyhow::{Context, Result};
use chrono::Local;
use colored::Colorize;
use std::io::{self, Write};

use crate::config::Config;
use crate::digitalocean::DoClient;
use crate::PowerAction;

pub async fn run(action: PowerAction) -> Result<()> {
    let config = Config::load()?;
    let client = DoClient::from_config()?;

    match action {
        PowerAction::On { name } => power_on(&config, &client, &name).await,
        PowerAction::Off { name, snapshot } => power_off(&config, &client, &name, snapshot).await,
        PowerAction::Reboot { name } => reboot(&config, &client, &name).await,
        PowerAction::Snapshot { name } => snapshot(&config, &client, &name).await,
    }
}

async fn power_on(config: &Config, client: &DoClient, name: &str) -> Result<()> {
    let droplet = config
        .get_droplet(name)
        .with_context(|| format!("Droplet '{}' not found in config", name))?;

    println!("{} Powering on {}...", "→".blue(), name.bold());
    let action = client.power_on(droplet.id).await?;
    client.wait_for_action(droplet.id, action.id).await?;
    println!("{} {} is now {}", "✓".green(), name.bold(), "active".green());

    Ok(())
}

async fn power_off(config: &Config, client: &DoClient, name: &str, snapshot: bool) -> Result<()> {
    let droplet = config
        .get_droplet(name)
        .with_context(|| format!("Droplet '{}' not found in config", name))?;

    // Prompt for snapshot if not specified
    let do_snapshot = if snapshot {
        true
    } else {
        print!("Create snapshot before shutdown? [y/N]: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_lowercase().starts_with('y')
    };

    if do_snapshot {
        let snap_name = format!("{}-{}", name, Local::now().format("%Y%m%d-%H%M%S"));
        println!("{} Creating snapshot: {}...", "→".blue(), snap_name);
        let action = client.snapshot(droplet.id, &snap_name).await?;
        client.wait_for_action(droplet.id, action.id).await?;
        println!("{} Snapshot created", "✓".green());
    }

    println!("{} Powering off {}...", "→".blue(), name.bold());
    let action = client.power_off(droplet.id).await?;
    client.wait_for_action(droplet.id, action.id).await?;
    println!("{} {} is now {}", "✓".green(), name.bold(), "off".red());

    Ok(())
}

async fn reboot(config: &Config, client: &DoClient, name: &str) -> Result<()> {
    let droplet = config
        .get_droplet(name)
        .with_context(|| format!("Droplet '{}' not found in config", name))?;

    println!("{} Rebooting {}...", "→".blue(), name.bold());
    let action = client.reboot(droplet.id).await?;
    client.wait_for_action(droplet.id, action.id).await?;
    println!("{} {} rebooted", "✓".green(), name.bold());

    Ok(())
}

async fn snapshot(config: &Config, client: &DoClient, name: &str) -> Result<()> {
    let droplet = config
        .get_droplet(name)
        .with_context(|| format!("Droplet '{}' not found in config", name))?;

    let snap_name = format!("{}-{}", name, Local::now().format("%Y%m%d-%H%M%S"));
    println!("{} Creating snapshot: {}...", "→".blue(), snap_name);
    let action = client.snapshot(droplet.id, &snap_name).await?;
    client.wait_for_action(droplet.id, action.id).await?;
    println!("{} Snapshot created: {}", "✓".green(), snap_name);

    Ok(())
}
