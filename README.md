# Tdops

Tailscale + DigitalOcean droplet ops CLI.

Manage your DO droplets with Tailscale integration from one tool.

## Install

```bash
# Build from source
cargo build --release

# Copy to PATH
cp target/release/tdops.exe ~/.local/bin/
```

## Setup

```bash
# Initialize with your DO API token
tdops init
```

Creates config at `~/.config/tdops/config.toml` (Linux/Mac) or `%APPDATA%\tdops\config.toml` (Windows).

## Commands

### Status

```bash
# Show all droplets with status
tdops status

# List configured droplets
tdops list
```

### Power Control

```bash
# Power on
tdops power on my-droplet

# Power off (prompts for snapshot)
tdops power off my-droplet

# Power off with snapshot
tdops power off my-droplet --snapshot

# Reboot
tdops power reboot my-droplet

# Create snapshot only
tdops power snapshot my-droplet
```

### Tailscale Serve/Funnel

```bash
# Check serve status (local)
tdops serve status

# Check serve status on remote droplet
tdops serve status -d my-droplet

# Expose port on tailnet only
tdops serve start 3000

# Expose port on remote droplet
tdops serve start 8080 -d my-droplet

# Expose port publicly via Funnel
tdops serve funnel 443

# Reset serve/funnel config
tdops serve reset
```

## Config

Config file location:
- Windows: `C:\Users\<user>\AppData\Roaming\tdops\config.toml`
- Linux/Mac: `~/.config/tdops/config.toml`

Example:
```toml
do_token = "dop_v1_xxx"

[droplets.my-droplet]
id = 123456789
public_ip = "1.2.3.4"

[droplets.another-droplet]
id = 987654321
public_ip = "5.6.7.8"
```

## Requirements

- Rust 1.70+ (to build)
- DigitalOcean API token with droplet read/update permissions
- Tailscale installed (for serve/funnel commands)

## License

MIT
