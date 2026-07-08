use crate::version_info::get_version;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    current_version: String,
    latest_version: String,
}

// On-disk cache so the GitHub call is deduped ACROSS app relaunches. Each app
// launch is a fresh process (handlePlay ends in exit(0), the updater relaunch
// spawns a new exe), so an in-memory cache wouldn't survive — a burst of rapid
// relaunches used to fire one unauthenticated GitHub call each and blow the
// 60-req/hr limit (-> 403 -> "tag_name not found" error in the webview). With
// the cache, a whole hour of relaunches costs at most one real request.
#[derive(Serialize, Deserialize)]
struct UpdateCache {
    checked_at_secs: u64,
    latest_version: String,
}

const CACHE_TTL_SECS: u64 = 3600; // re-check GitHub at most once per hour
const REQUEST_TIMEOUT_SECS: u64 = 10;

// Per-user app-data dir, NOT the game's Display_Config_Resources folder: the
// game may live somewhere the user can't write (e.g. Program Files), and a
// failed cache write would silently bring the request storm back.
fn cache_path() -> std::path::PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("SS-Dat-Info").join("update_check_cache.json")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn read_cache() -> Option<UpdateCache> {
    serde_json::from_str(&std::fs::read_to_string(cache_path()).ok()?).ok()
}

fn write_cache(latest_version: &str) {
    let cache = UpdateCache {
        checked_at_secs: now_secs(),
        latest_version: latest_version.to_string(),
    };
    let path = cache_path();
    if let Some(dir) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("update cache: create dir {} failed: {e}", dir.display());
            return;
        }
    }
    match serde_json::to_string(&cache) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                eprintln!("update cache: write {} failed: {e}", path.display());
            }
        }
        Err(e) => eprintln!("update cache: serialize failed: {e}"),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = get_version().to_string();

    // Fresh cache -> serve it, no network. Collapses relaunch storms to ~1/hr.
    if let Some(c) = read_cache() {
        if now_secs().saturating_sub(c.checked_at_secs) < CACHE_TTL_SECS {
            return Ok(UpdateInfo {
                current_version,
                latest_version: c.latest_version,
            });
        }
    }

    match fetch_latest_version().await {
        Ok(latest) => {
            write_cache(&latest);
            Ok(UpdateInfo {
                current_version,
                latest_version: latest,
            })
        }
        // NEVER surface an Err: the frontend turns a rejected command into an
        // error dialog that reads like a crash. A failed/rate-limited update
        // check is not user-facing — degrade gracefully. Prefer a stale cached
        // version (so a real pending update still shows); otherwise report
        // "no update" (latest == current).
        Err(e) => {
            eprintln!("update check failed, degrading gracefully: {e}");
            let latest_version = read_cache()
                .map(|c| c.latest_version)
                .unwrap_or_else(|| current_version.clone());
            Ok(UpdateInfo {
                current_version,
                latest_version,
            })
        }
    }
}

async fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
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
        .ok_or(format!("\"tag_name\" not found in response: {:?}", response))?
        .trim_start_matches('v')
        .to_string();

    Ok(version)
}
