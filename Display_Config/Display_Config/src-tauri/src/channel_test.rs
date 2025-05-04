use serde::Serialize;
use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Started {
        url: &'a str,
        download_id: usize,
        content_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        download_id: usize,
        chunk_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Finished { download_id: usize },
}

#[tauri::command]
pub fn channel_test(on_event: Channel<DownloadEvent>) {
    let url = "https://example.com/file.zip";
    let content_length = 1000;
    let download_id = 1;

    on_event
        .send(DownloadEvent::Started {
            url,
            download_id,
            content_length,
        })
        .unwrap();

    for chunk_length in [15, 150, 35, 500, 300] {
        on_event
            .send(DownloadEvent::Progress {
                download_id,
                chunk_length,
            })
            .unwrap();
    }

    on_event
        .send(DownloadEvent::Finished { download_id })
        .unwrap();
}
