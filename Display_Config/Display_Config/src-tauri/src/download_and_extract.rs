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

mod install;

pub fn cleanup_updates() {
    install::cleanup(&get_supreme_folder());
}

pub fn exit_if_not_installing(code: i32) {
    install::when_not_installing(|| std::process::exit(code));
}
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
    Token {
        token: String,
    },

    #[serde(rename_all = "camelCase")]
    DownloadCancelled,
    Installing,
}
#[tauri::command]
#[specta::specta]
pub async fn download_and_extract(
    url: String,
    on_event: Channel<DownloadEvent>,
) -> Result<DownloadResult, String> {
    let cancellation_registry = CancellationRegistry::instance();
    let (id, token) = cancellation_registry.create_and_register_task();
    let result = async {
        on_event.send(DownloadEvent::Token {
            token: id.to_string(),
        })?;
        do_download_and_extract(url, on_event, token, id).await
    }
    .await;
    cancellation_registry.remove_task(&id);
    result.map_err(|e: anyhow::Error| {
        error!("Update failed: {e:#}");
        format!("Update failed: {e:#}")
    })
}

#[derive(Serialize, specta::Type)]
pub struct DownloadResult {
    installed: bool,
}

async fn do_download_and_extract(
    url: String,
    on_event: Channel<DownloadEvent>,
    token: Arc<AtomicBool>,
    id: uuid::Uuid,
) -> Result<DownloadResult> {
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .read_timeout(std::time::Duration::from_secs(15))
        .build()?;
    let response = client
        .get(url)
        .header("User-Agent", "SS-Dat-Info-App")
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Download error: {}", response.status());
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

    let installed = install::install_zip(
        buffer,
        &get_supreme_folder(),
        || is_task_cancelled(&token),
        || {
            // Close cancellation before replacing files. Any accepted cancellation wins.
            CancellationRegistry::instance().remove_task(&id);
            if is_task_cancelled(&token) {
                return Ok(false);
            }
            on_event.send(DownloadEvent::Installing)?;
            Ok(true)
        },
    )?;
    if !installed {
        on_event.send(DownloadEvent::DownloadCancelled)?;
    }
    info!("Update installation completed: {installed}");
    Ok(DownloadResult { installed })
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
