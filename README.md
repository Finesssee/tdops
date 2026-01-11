# Tdops

> Tailscale + DigitalOcean droplet operations CLI

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A unified CLI for managing DigitalOcean droplets with Tailscale integration. Power control, snapshots, and Tailscale Serve/Funnel from one tool.

## Features

- **Status Dashboard** — View all droplets with status, IPs in a table
- **Power Control** — On/off/reboot with optional snapshot before shutdown
- **Snapshots** — Create on-demand or automatic pre-shutdown backups
- **Tailscale Serve** — Expose ports on your tailnet
- **Tailscale Funnel** — Expose ports publicly via HTTPS
- **Auto-config** — Fetches droplet info from DO API on init

## Installation

### From source

```bash
git clone https://github.com/Finesssee/tdops.git
cd tdops
cargo build --release

# Add to PATH
cp target/release/tdops ~/.local/bin/
```

### Requirements

- Rust 1.70+
- [DigitalOcean API token](https://cloud.digitalocean.com/account/api/tokens) (read + update permissions)
- [Tailscale](https://tailscale.com/) installed (for serve/funnel commands)

## Quick Start

```bash
# Initialize with your DO API token
tdops init

# View all droplets
tdops status

# Power on a droplet
tdops power on my-droplet

# Power off with snapshot prompt
tdops power off my-droplet
```

## Usage

### Status

```bash
# Show all droplets in a table
tdops status

# List configured droplets from config
tdops list
```

### Power Control

```bash
tdops power on <droplet>        # Power on
tdops power off <droplet>       # Power off (prompts for snapshot)
tdops power off <droplet> -s    # Power off with snapshot
tdops power reboot <droplet>    # Reboot
tdops power snapshot <droplet>  # Create snapshot only
```

### Tailscale Serve/Funnel

```bash
# Check serve status
tdops serve status
tdops serve status -d <droplet>

# Expose port on tailnet only
tdops serve start <port>
tdops serve start <port> -d <droplet>

# Expose port publicly via Funnel
tdops serve funnel <port>
tdops serve funnel <port> -d <droplet>

# Reset all serve/funnel config
tdops serve reset
```

## Configuration

Config is stored at:
- **Windows:** `%APPDATA%\tdops\config.toml`
- **Linux/macOS:** `~/.config/tdops/config.toml`

### Example

```toml
do_token = "dop_v1_..."

[droplets.web-server]
id = 123456789
public_ip = "1.2.3.4"
tailscale_ip = "100.x.x.x"  # optional

[droplets.database]
id = 987654321
public_ip = "5.6.7.8"
```

Run `tdops init` to auto-generate this from your DO account.

## How It Works

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│   tdops     │────▶│  DigitalOcean    │     │  Droplets   │
│   CLI       │     │  API             │────▶│  (VPS)      │
└─────────────┘     └──────────────────┘     └─────────────┘
      │                                            │
      │             ┌──────────────────┐           │
      └────────────▶│  Tailscale       │◀──────────┘
                    │  (SSH/Serve)     │
                    └──────────────────┘
```

- Power commands use the DigitalOcean API
- Serve/Funnel commands use `tailscale` CLI via SSH
- Status fetches live data from DO API

## Contributing

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/foo`)
3. Commit changes (`git commit -m 'Add foo'`)
4. Push (`git push origin feature/foo`)
5. Open a Pull Request

## License

MIT
