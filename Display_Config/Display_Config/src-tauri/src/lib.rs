use tauri::Manager;

mod close_others;
mod config_parser;
mod config_writer;
mod detail_config;
mod file_commands;
mod get_log_data;
mod inject;
mod language;
mod path_util;
mod player_types;
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
            file_commands::open_player_types,
            close_others::close_others,
            get_log_data::get_log_data,
            language::write_language,
            detail_config::write_detail_config,
            detail_config::read_detail_config,
            player_types::set_first_player_type,
            player_types::get_first_player_type,
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
