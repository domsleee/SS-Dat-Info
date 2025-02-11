use tauri::Manager;

mod close_others;
mod file_commands;
mod inject;
mod path_util;
mod rd_config;

// fixme: is this really needed?
#[tauri::command]
fn kill_exit_1() -> String {
    std::process::exit(1);
}

// crazy hack: https://github.com/tauri-apps/tauri/issues/1564
#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            show_window,
            inject::run_inject,
            rd_config::read_rd_config,
            rd_config::write_rd_config,
            file_commands::open_log_file,
            close_others::close_others,
            kill_exit_1
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let package_info = app.package_info();
            window.set_title(&format!(
                "Supreme Snowboarding Config {}",
                package_info.version
            ))?;
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    std::process::exit(1);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
