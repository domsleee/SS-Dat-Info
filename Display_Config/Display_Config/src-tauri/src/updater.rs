use crate::version_info::get_version;
use serde::{Deserialize, Serialize};
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
    checked_at_secs: u64, // last SUCCESSFUL check
    // Last attempt, success OR failure. Failures don't refresh checked_at (a
    // transient blip must not hide a pending update for a whole TTL), but they
    // do back off retries — without this, a cold start while rate-limited
    // would fire one request per relaunch until the first success.
    #[serde(default)]
    last_attempt_secs: u64,
    latest_version: String,
}

const CACHE_TTL_SECS: u64 = 3600; // re-check GitHub at most once per hour
const FAILURE_RETRY_SECS: u64 = 300; // after a failed check, back off 5 min
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

fn write_cache(cache: &UpdateCache) {
    let path = cache_path();
    if let Err(e) = write_cache_at(&path, cache) {
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
pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = get_version().to_string();

    // Serve the cache without touching the network while the last success is
    // fresh, or while backing off after a recent failed attempt. Collapses
    // relaunch storms to ~1 request/hr (worst case 12/hr while failing).
    if let Some(c) = read_cache() {
        let now = now_secs();
        let success_fresh = now.saturating_sub(c.checked_at_secs) < CACHE_TTL_SECS;
        let backing_off = now.saturating_sub(c.last_attempt_secs) < FAILURE_RETRY_SECS;
        if success_fresh || backing_off {
            return Ok(UpdateInfo::new(current_version, c.latest_version));
        }
    }

    match fetch_latest_version().await {
        Ok(latest) => {
            write_cache(&UpdateCache {
                checked_at_secs: now_secs(),
                last_attempt_secs: now_secs(),
                latest_version: latest.clone(),
            });
            Ok(UpdateInfo::new(current_version, latest))
        }
        // NEVER surface an Err: the frontend turns a rejected command into an
        // error dialog that reads like a crash. A failed/rate-limited update
        // check is not user-facing — degrade gracefully. Prefer a stale cached
        // version (so a real pending update still shows); otherwise report
        // "no update" (latest == current).
        Err(e) => {
            eprintln!("update check failed, degrading gracefully: {e}");
            let prev = read_cache();
            let latest_version = prev
                .as_ref()
                .map(|c| c.latest_version.clone())
                .unwrap_or_else(|| current_version.clone());
            // Stamp the attempt (keeping the old success time) so the next
            // few relaunches back off instead of re-firing the request.
            write_cache(&UpdateCache {
                checked_at_secs: prev.map(|c| c.checked_at_secs).unwrap_or(0),
                last_attempt_secs: now_secs(),
                latest_version: latest_version.clone(),
            });
            Ok(UpdateInfo::new(current_version, latest_version))
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
        .ok_or(format!(
            "\"tag_name\" not found in response: {:?}",
            response
        ))?
        .trim_start_matches('v')
        .to_string();

    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_offers_newer_versions() {
        for (current, latest, offered) in [
            ("0.4.5", "0.4.4", false),
            ("0.4.5", "0.4.5", false),
            ("0.4.5", "0.4.6", true),
            ("0.4.9", "0.4.10", true),
            ("0.4.5", "0.4.6-rc.1", true),
            ("0.4.5", "0.4.5-rc.1", false),
            ("0.4.5-rc.1", "0.4.5", true),
            ("0.4.5+build.1", "0.4.5+build.2", false),
            ("0.4.5", "invalid", false),
            ("0.4.5", "", false),
        ] {
            let info = UpdateInfo::new(current.into(), latest.into());
            assert_eq!(info.current_version, current);
            assert_eq!(info.latest_version, if offered { latest } else { current });
        }
    }

    #[test]
    fn concurrent_writes_publish_complete_caches() {
        let dir = std::env::temp_dir().join(format!("ss-update-{}", uuid::Uuid::new_v4()));
        let path = dir.join("cache.json");
        let barrier = std::sync::Barrier::new(8);
        std::thread::scope(|scope| {
            for writer in 0..8 {
                let path = &path;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    for attempt in 0..32 {
                        let stamp = writer * 32 + attempt;
                        write_cache_at(
                            path,
                            &UpdateCache {
                                checked_at_secs: stamp,
                                last_attempt_secs: stamp,
                                latest_version: format!("0.4.{stamp}"),
                            },
                        )
                        .unwrap();
                        let cache: UpdateCache =
                            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
                        assert_eq!(cache.checked_at_secs, cache.last_attempt_secs);
                        assert_eq!(
                            cache.latest_version,
                            format!("0.4.{}", cache.checked_at_secs)
                        );
                    }
                });
            }
        });
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn failed_publish_removes_temporary_file() {
        let dir = std::env::temp_dir().join(format!("ss-update-{}", uuid::Uuid::new_v4()));
        let path = dir.join("cache.json");
        std::fs::create_dir_all(&path).unwrap();
        assert!(
            write_cache_at(
                &path,
                &UpdateCache {
                    checked_at_secs: 1,
                    last_attempt_secs: 1,
                    latest_version: "0.4.5".into(),
                },
            )
            .is_err()
        );
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        std::fs::remove_dir(path).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
