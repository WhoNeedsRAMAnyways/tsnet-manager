

use std::process::Command;


pub fn list_profiles() -> Result<String, String> {
    let output = Command::new("tailscale")
        .args(["switch", "--list"])
        .output().map_err(|e| e.to_string())?;

    if!output.status.success(){
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}