use crate::version_info::get_version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    current_version: String,
    latest_version: String,
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = get_version();
    let latest_version = match fetch_latest_version().await {
        Ok(version) => version,
        Err(e) => return Err(format!("Failed to fetch latest version: {}", e)),
    };

    Ok(UpdateInfo {
        current_version: current_version.to_string(),
        latest_version,
    })
}

async fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = serde_json::from_str::<serde_json::Value>(
        &client
            .get("https://api.github.com/repos/domsleee/SS-Dat-Info/releases/latest")
            .header("User-Agent", "SS-Dat-Info-App")
            .send()
            .await?
            .text()
            .await?,
    )?;

    // Extract the tag_name which contains version (usually in format "v1.2.3")
    let version = response["tag_name"]
        .as_str()
        .unwrap_or("0.0.0")
        .trim_start_matches('v')
        .to_string();

    Ok(version)
}
