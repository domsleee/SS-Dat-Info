use crate::version_info::get_version;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    current_version: String,
    latest_version: String,
}

impl UpdateInfo {
    fn new(current_version: String, latest_version: String) -> Self {
        let newer = match (
            semver::Version::parse(&current_version),
            semver::Version::parse(&latest_version),
        ) {
            (Ok(current), Ok(latest)) => latest.cmp_precedence(&current).is_gt(),
            _ => false,
        };
        // Both update controls interpret unequal versions as an available update.
        Self {
            latest_version: if newer {
                latest_version
            } else {
                current_version.clone()
            },
            current_version,
        }
    }
}

// Persist checks across launcher restarts to avoid GitHub's rate limit.
#[derive(Serialize, Deserialize)]
struct UpdateCache {
    checked_at_secs: u64, // Last successful check.
    #[serde(default)]
    last_attempt_secs: u64, // Includes failed checks.
    latest_version: String,
}

impl UpdateCache {
    fn can_reuse(&self, now: u64) -> bool {
        let recent = |stamp, ttl| stamp != 0 && now.checked_sub(stamp).is_some_and(|age| age < ttl);
        recent(self.checked_at_secs, CACHE_TTL_SECS)
            || recent(self.last_attempt_secs, FAILURE_RETRY_SECS)
    }
}

const CACHE_TTL_SECS: u64 = 3600;
const FAILURE_RETRY_SECS: u64 = 300;
const REQUEST_TIMEOUT_SECS: u64 = 10;

// The game directory may not be writable.
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

fn read_cache(path: &Path) -> Option<UpdateCache> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

fn write_cache(path: &Path, cache: &UpdateCache) {
    if let Err(e) = write_cache_at(path, cache) {
        eprintln!("update cache: write {} failed: {e}", path.display());
    }
}

fn write_cache_at(path: &Path, cache: &UpdateCache) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_vec(cache)?;
    // Each writer owns a separate file until the complete cache is published.
    let tmp = path.with_extension(format!("{}.tmp", uuid::Uuid::new_v4()));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)?;
    let result = file.write_all(&json);
    drop(file);
    let result = result.and_then(|()| std::fs::rename(&tmp, path));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

#[tauri::command]
#[specta::specta]
pub async fn check_for_updates(force: bool) -> Result<UpdateInfo, String> {
    check_updates(force, &cache_path(), async {
        fetch_latest_version().await.map_err(|e| e.to_string())
    })
    .await
}

async fn check_updates(
    force: bool,
    path: &Path,
    fetch: impl Future<Output = Result<String, String>>,
) -> Result<UpdateInfo, String> {
    let current_version = get_version();
    let mut cache = read_cache(path).unwrap_or_else(|| UpdateCache {
        checked_at_secs: 0,
        last_attempt_secs: 0,
        latest_version: current_version.clone(),
    });

    if !force && cache.can_reuse(now_secs()) {
        return Ok(UpdateInfo::new(current_version, cache.latest_version));
    }

    let result = fetch.await;
    let now = now_secs();
    match &result {
        Ok(latest) => {
            cache.checked_at_secs = now;
            cache.latest_version = latest.clone();
        }
        Err(e) => {
            eprintln!("update check failed: {e}");
            // Another launcher may have published a successful check while we waited.
            cache = read_cache(path).unwrap_or(cache);
        }
    }
    cache.last_attempt_secs = now;
    write_cache(path, &cache);
    if force {
        result.map_err(|e| format!("Could not check for updates: {e}"))?;
    }
    Ok(UpdateInfo::new(current_version, cache.latest_version))
}

async fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
    let body = client
        .get("https://api.github.com/repos/domsleee/SS-Dat-Info/releases/latest")
        .header("User-Agent", "SS-Dat-Info-App")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let response: serde_json::Value = serde_json::from_str(&body)?;

    let version = response["tag_name"]
        .as_str()
        .ok_or_else(|| format!("\"tag_name\" not found in response: {response:?}"))?
        .trim_start_matches('v')
        .to_string();

    Ok(version)
}

#[cfg(test)]
mod tests;
