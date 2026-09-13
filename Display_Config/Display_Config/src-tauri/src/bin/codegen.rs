use std::process::Command;

use tauri_app_lib::get_tauri_specta_builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_tauri_specta_builder();
    let path = std::env::current_dir().unwrap().join("../src/bindings.ts");
    let output = Command::new("bun")
        .arg("run")
        .arg("eslint")
        .arg("--fix")
        .arg(path.canonicalize().unwrap())
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "eslint --fix failed:\n{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(())
}
