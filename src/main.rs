mod commands;
mod config;
mod digitalocean;
mod tailscale;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{power, serve, status};

#[derive(Parser)]
#[command(name = "tdops")]
#[command(about = "Tailscale + DigitalOcean droplet ops CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show status of all droplets
    Status,
    /// Power control (on/off/reboot/snapshot)
    Power {
        #[command(subcommand)]
        action: PowerAction,
    },
    /// Expose ports via Tailscale Serve/Funnel
    Serve {
        #[command(subcommand)]
        action: ServeAction,
    },
    /// List configured droplets
    List,
    /// Initialize config with DO API token
    Init,
}

#[derive(Subcommand)]
enum PowerAction {
    /// Power on a droplet
    On { name: String },
    /// Power off a droplet (prompts for snapshot)
    Off {
        name: String,
        #[arg(short, long)]
        snapshot: bool,
    },
    /// Reboot a droplet
    Reboot { name: String },
    /// Create a snapshot
    Snapshot { name: String },
}

#[derive(Subcommand)]
enum ServeAction {
    /// Show current serve/funnel status
    Status { droplet: Option<String> },
    /// Expose port on tailnet only
    Start {
        port: u16,
        #[arg(short, long)]
        droplet: Option<String>,
    },
    /// Expose port publicly via Funnel
    Funnel {
        port: u16,
        #[arg(short, long)]
        droplet: Option<String>,
    },
    /// Reset serve/funnel config
    Reset { droplet: Option<String> },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => status::run().await?,
        Commands::Power { action } => power::run(action).await?,
        Commands::Serve { action } => serve::run(action).await?,
        Commands::List => status::list().await?,
        Commands::Init => config::init().await?,
    }

    Ok(())
}
