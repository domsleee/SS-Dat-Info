use cancellation_registry::CancellationRegistry;
use tauri::Manager;
use tauri_specta::ErrorHandlingMode;

mod cancellation_registry;
mod close_others;
mod config_parser;
mod config_writer;
mod detail_config;
mod download_and_extract;
mod file_commands;
mod get_log_data;
mod inject;
mod language;
mod path_util;
mod player_types;
mod rd_config;
mod relaunch;
mod transfer_stats;
mod updater;
mod version_info;
mod performance;

// fixme: is this really needed?
#[tauri::command]
#[specta::specta]
fn kill_exit_1() -> String {
    std::process::exit(1);
}

// crazy hack: https://github.com/tauri-apps/tauri/issues/1564
#[tauri::command]
#[specta::specta]
fn show_window(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = get_tauri_specta_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            app.manage(CancellationRegistry::default());
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

pub fn get_tauri_specta_builder() -> tauri_specta::Builder {
    let builder = tauri_specta::Builder::new()
        .commands(tauri_specta::collect_commands![
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
            updater::check_for_updates,
            download_and_extract::download_and_extract,
            download_and_extract::cancel_download,
            version_info::get_version,
            relaunch::relaunch,
            performance::log_startup_time,
            kill_exit_1
        ])
        .error_handling(ErrorHandlingMode::Throw);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                .header("/* eslint-disable @typescript-eslint/no-explicit-any, @typescript-eslint/no-unused-vars, @typescript-eslint/ban-ts-comment */\n// @ts-nocheck")
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                .formatter(specta_typescript::formatter::eslint),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    builder
}
