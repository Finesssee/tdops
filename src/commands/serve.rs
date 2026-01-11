use anyhow::Result;
use colored::Colorize;

use crate::tailscale;
use crate::ServeAction;

pub async fn run(action: ServeAction) -> Result<()> {
    match action {
        ServeAction::Status { droplet } => status(droplet.as_deref()).await,
        ServeAction::Start { port, droplet } => start(port, droplet.as_deref()).await,
        ServeAction::Funnel { port, droplet } => funnel(port, droplet.as_deref()).await,
        ServeAction::Reset { droplet } => reset(droplet.as_deref()).await,
    }
}

async fn status(droplet: Option<&str>) -> Result<()> {
    let target = droplet.unwrap_or("local");
    println!("{} Serve status for {}:\n", "→".blue(), target.bold());

    let output = tailscale::serve_status(droplet)?;
    if output.trim().is_empty() || output.contains("No serve config") {
        println!("  (no serve/funnel configured)");
    } else {
        println!("{}", output);
    }

    Ok(())
}

async fn start(port: u16, droplet: Option<&str>) -> Result<()> {
    let target = droplet.unwrap_or("local");
    println!(
        "{} Exposing port {} on tailnet ({})",
        "→".blue(),
        port.to_string().bold(),
        target
    );

    let output = tailscale::serve_start(droplet, port)?;
    println!("{}", output);
    println!("{} Serve started on port {}", "✓".green(), port);

    Ok(())
}

async fn funnel(port: u16, droplet: Option<&str>) -> Result<()> {
    let target = droplet.unwrap_or("local");
    println!(
        "{} Exposing port {} publicly via Funnel ({})",
        "→".blue(),
        port.to_string().bold(),
        target
    );

    let output = tailscale::funnel_start(droplet, port)?;
    println!("{}", output);
    println!("{} Funnel started on port {}", "✓".green(), port);

    Ok(())
}

async fn reset(droplet: Option<&str>) -> Result<()> {
    let target = droplet.unwrap_or("local");
    println!("{} Resetting serve/funnel on {}...", "→".blue(), target);

    tailscale::serve_reset(droplet)?;
    println!("{} Serve/funnel reset", "✓".green());

    Ok(())
}
