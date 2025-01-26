use std::env;
use std::path::{Path, PathBuf};

pub fn get_supreme_folder() -> PathBuf {
    let exe_dir = env::current_exe()
        .map(|p| p.parent().unwrap_or(Path::new("")).to_path_buf())
        .unwrap_or_default();

    if exe_dir.join("Supreme_Game.dll").exists() {
        exe_dir
    } else {
        PathBuf::from(r"C:\Games\Supreme")
    }
}
