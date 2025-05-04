use std::process;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn relaunch(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().hide().unwrap();

    let shell = app.shell();
    let result = shell
        .command(
            std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        )
        .env("TAURI_RELAUNCHED", "1") // Mark as relaunched to prevent recursion
        .output()
        .await;

    process::exit(result.unwrap().status.code().unwrap_or(1));
}
