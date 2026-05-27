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

pub fn fetch_crate(name: &str) -> Result<CrateInfo, String> {
    let url = format!("https://crates.io/api/v1/crates/{}", name);
    
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .header("User-Agent", "tsnp/0.1.0")
        .send()
        .map_err(|e| format!("HTTP error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Crate '{}' not found", name));
    }
    
    let info: CrateInfo = response
        .json()
        .map_err(|e| format!("JSON error: {}", e))?;
    
    Ok(info)
}
