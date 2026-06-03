use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrateInfo {
    #[serde(rename = "crate")]
    pub crate_: CrateData,
}

#[derive(Deserialize)]
pub struct CrateData {
    pub repository: Option<String>,
}

pub fn fetch_crate(name: &str) -> Result<CrateInfo> {
    let url = format!("https://crates.io/api/v1/crates/{}", name);
    
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("User-Agent", format!("tsnp/{}", env!("CARGO_PKG_VERSION")))
        .send()
        .context("Failed to send HTTP request to crates.io")?;
    
    if !response.status().is_success() {
        anyhow::bail!("Crate '{}' not found on crates.io (status: {})", name, response.status());
    }
    
    let info: CrateInfo = response
        .json()
        .context("Failed to parse crates.io JSON response")?;
    
    Ok(info)
}
