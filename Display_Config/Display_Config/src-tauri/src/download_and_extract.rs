use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::cancellation_registry::{CancellationRegistry, is_task_cancelled};
use crate::{path_util::get_supreme_folder, transfer_stats::TransferStats};
use anyhow::Context;
use anyhow::Result;
use futures_util::TryStreamExt;
use log::{error, info};
use serde::Serialize;
use tauri::ipc::Channel;
#[derive(Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    DownloadProgress {
        progress: u64,
        progress_total: u64,
        total: u64,
        transfer_speed: u64,
    },

    #[serde(rename_all = "camelCase")]
    Token { token: String },

    #[serde(rename_all = "camelCase")]
    DownloadCancelled,
}
#[tauri::command]
#[specta::specta]
pub async fn download_and_extract(
    url: String,
    on_event: Channel<DownloadEvent>,
) -> Result<DownloadResult, String> {
    let cancellation_registry = CancellationRegistry::instance();
    let (id, token) = cancellation_registry.create_and_register_task();
    on_event
        .send(DownloadEvent::Token {
            token: id.to_string(),
        })
        .expect("Can send token");
    match do_download_and_extract(url, on_event, token).await {
        Ok(result) => {
            cancellation_registry.remove_task(&id);
            Ok(result)
        }
        Err(e) => {
            error!("Error: {e:?}");
            cancellation_registry.remove_task(&id);
            Err(format!("download_and_extract error: {e:?}"))
        }
    }
}

#[derive(Serialize, specta::Type)]
pub struct DownloadResult {
    installed: bool,
}

async fn do_download_and_extract(
    url: String,
    on_event: Channel<DownloadEvent>,
    token: Arc<AtomicBool>,
) -> Result<DownloadResult, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "SS-Dat-Info-App")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Download error: {}", response.status()).into());
    }
    let total = response.content_length().unwrap_or(0);

    let mut buffer = Vec::new();
    let mut stream = response.bytes_stream();
    let mut stats = TransferStats::default();
    while let Some(chunk) = stream.try_next().await.context("failed to try_next")? {
        if is_task_cancelled(&token) {
            on_event.send(DownloadEvent::DownloadCancelled)?;
            return Ok(DownloadResult { installed: false });
        }
        stats.record_chunk_transfer(chunk.len());
        buffer.extend_from_slice(&chunk);
        on_event.send(DownloadEvent::DownloadProgress {
            progress: chunk.len() as u64,
            progress_total: stats.total_transferred,
            total,
            transfer_speed: stats.transfer_speed,
        })?;
    }

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(buffer))?;

    let temp_dir = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
    info!(
        "Extracting files to {}, temp_folder {temp_dir:?}...",
        get_supreme_folder().display()
    );
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = get_supreme_folder().join(file.name());
        if is_task_cancelled(&token) {
            on_event.send(DownloadEvent::DownloadCancelled)?;
            return Ok(DownloadResult { installed: false });
        }

        if file.name().ends_with('/') {
            if !outpath.exists() {
                std::fs::create_dir_all(&outpath).context("Failed to create_dir_all")?;
            }
        } else {
            if let Some(parent) = outpath.parent()
                && !parent.exists() {
                    std::fs::create_dir_all(parent).context("Failed to create_dir_all")?;
                }

            if outpath.exists() {
                let temp_path = temp_dir.join(file.name());
                if let Some(parent) = temp_path.parent()
                    && !parent.exists() {
                        std::fs::create_dir_all(parent).context("Failed to create_dir_all")?;
                    }
                std::fs::rename(&outpath, &temp_path).context("Failed to rename")?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile).context("failed to copy")?;
        }
    }
    info!("Extraction complete.");
    Ok(DownloadResult { installed: true })
}

#[tauri::command]
#[specta::specta]
pub fn cancel_download(id: String) -> bool {
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return false,
    };
    let cancellation_registry = CancellationRegistry::instance();
    cancellation_registry.cancel_task(&uuid)
}
