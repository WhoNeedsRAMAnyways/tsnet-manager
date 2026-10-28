

use std::process::Command;


pub struct Profile {
    pub id: String,
    pub tailnet: String,
    pub account: String,
    pub active: bool,
}



pub fn list_profiles() -> Result<String, String> {
    let output = Command::new("tailscale")
        .args(["switch", "--list"])
        .output().map_err(|e| e.to_string())?;

    if !output.status.success(){
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}


pub fn parse_profiles(text: &str) -> Vec<Profile> {

    text.lines()
        .skip(1) //Skip the header row
        .filter_map(|line| {
            let mut cols = line.split_whitespace();
            let id = cols.next()?;
            let tailnet = cols.next()?;
            let account = cols.next()?;



            Some(Profile {
                id: id.to_owned(),
                tailnet: tailnet.to_owned(),
                account: account.trim_end_matches('*').to_owned(),
                active: account.ends_with('*'),
            })
        })
    .collect()
}


pub fn switch_profile(id: &str) -> Result<String, String>{

    let output = Command::new("tailscale")
        .args(["switch", id])
        .output().map_err(|e| e.to_string())?;

    if !output.status.success(){
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}
