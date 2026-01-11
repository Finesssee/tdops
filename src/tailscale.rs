use anyhow::Result;
use std::process::Command;

pub fn status() -> Result<String> {
    let output = Command::new("tailscale")
        .args(["status", "--json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn serve_status(host: Option<&str>) -> Result<String> {
    run_tailscale_cmd(host, &["serve", "status"])
}

pub fn serve_start(host: Option<&str>, port: u16) -> Result<String> {
    run_tailscale_cmd(host, &["serve", "--bg", "--yes", &port.to_string()])
}

pub fn funnel_start(host: Option<&str>, port: u16) -> Result<String> {
    run_tailscale_cmd(host, &["funnel", "--bg", "--yes", &port.to_string()])
}

pub fn serve_reset(host: Option<&str>) -> Result<String> {
    let _ = run_tailscale_cmd(host, &["serve", "reset"]);
    run_tailscale_cmd(host, &["funnel", "reset"])
}

fn run_tailscale_cmd(host: Option<&str>, args: &[&str]) -> Result<String> {
    let output = match host {
        Some(h) => {
            let ssh_cmd = format!("tailscale {}", args.join(" "));
            Command::new("ssh")
                .args(["root@".to_string() + h, ssh_cmd])
                .output()?
        }
        None => Command::new("tailscale").args(args).output()?,
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() && stdout.is_empty() {
        Ok(stderr)
    } else {
        Ok(stdout)
    }
}
