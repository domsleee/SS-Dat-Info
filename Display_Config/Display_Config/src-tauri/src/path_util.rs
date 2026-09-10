use std::env;
use std::path::{Path, PathBuf};

pub fn get_supreme_folder() -> PathBuf {
    let exe_dir = env::current_exe()
        .map(|p| p.parent().unwrap_or(Path::new("")).to_path_buf())
        .unwrap_or_default();

    if looks_like_supreme_folder(&exe_dir) {
        exe_dir
    } else if cfg!(debug_assertions) {
        PathBuf::from(r"C:\Games\Supreme2")
    } else {
        exe_dir
    }
}

fn looks_like_supreme_folder(path: &Path) -> bool {
    path.join("Data").is_dir()
        && (path.join("Supreme.exe").exists()
            || path.join("Supreme_v1.035.exe").exists()
            || path.join("Display_Config_Resources").is_dir())
}
