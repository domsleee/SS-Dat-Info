use std::process::Command;

use tauri_app_lib::get_tauri_specta_builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_tauri_specta_builder();
    let path = std::env::current_dir().unwrap().join("../src/bindings.ts");
    Command::new("bun")
        .arg("run")
        .arg("eslint")
        .arg("--fix")
        .arg(path.canonicalize().unwrap())
        .output()?;
    Ok(())
}
