use std::{os::windows::process::CommandExt, path::PathBuf, process::Command, time::Duration};

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrainerSettings {
    #[specta(rename = "use4xFonts")]
    pub use4x_fonts: bool,
    pub change_fov: bool,
    pub fov_width: Option<i32>,
    pub fov_height: Option<i32>,
    pub enable_logging: bool,
    pub make_ghosts_opaque: bool,
    pub match_ghost_sounds_to_character: bool,
    pub disable_direct_input: bool,
    pub enable_custom_controls: bool,
    pub hide_blinking_r: bool,
    pub show_replay_speed: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn run_inject(trainer_settings: TrainerSettings) -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");
    let settings_path = display_config_resources.join("Display_Config_Helper.json");
    let settings_file = std::fs::File::create(settings_path).unwrap();

    let log_path = get_display_config_helper_log_path();
    if log_path.exists() {
        std::fs::remove_file(&log_path).expect("Failed to remove log file");
    }

    let writer = std::io::BufWriter::new(&settings_file);
    serde_json::to_writer_pretty(writer, &trainer_settings).unwrap();

    let injector_path = display_config_resources.join("Injector.exe");
    let dll_path = display_config_resources.join("Display_Config_Helper.dll");
    let status = Command::new(injector_path)
        .arg(&dll_path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW (https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
        .current_dir(&display_config_resources)
        .status()
        .map_err(|err| format!("Failed to spawn process: {err}"))?;

    if !status.success() {
        return Err("Injector.exe failed.\nDid you run using Supreme.exe?".to_string());
    }

    wait_for_finished_log(&log_path)
}

pub fn get_display_config_helper_log_path() -> PathBuf {
    get_display_config_resources_path().join("Display_Config_Helper.log")
}

pub fn get_display_config_resources_path() -> PathBuf {
    let supreme_folder = get_supreme_folder();
    supreme_folder.join("Display_Config_Resources")
}

/// Inject TAS_Helper.dll into the running Supreme.exe process.
/// Uses the shared Injector.exe in Display_Config_Resources, with TAS payload
/// files located under Display_Config_Resources/TAS/.
#[tauri::command]
#[specta::specta]
pub async fn run_tas_inject() -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");
    let tas_folder = display_config_resources.join("TAS");
    let injector_path = display_config_resources.join("Injector.exe");
    let dll_path = tas_folder.join("TAS_Helper.dll");

    if !injector_path.exists() {
        return Err(format!("Injector not found at {}", injector_path.display()));
    }
    if !dll_path.exists() {
        return Err(format!(
            "TAS_Helper.dll not found at {}",
            dll_path.display()
        ));
    }

    let status = Command::new(&injector_path)
        .arg(&dll_path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .current_dir(&tas_folder)
        .status()
        .map_err(|err| format!("Failed to spawn Injector: {err}"))?;

    if !status.success() {
        return Err("Injector.exe failed.\nIs Supreme.exe running?".to_string());
    }

    // Injector.exe currently exits 0 even when CreateRemoteThread or
    // LoadLibraryA inside the target failed — the Inject sub-routine
    // logs and returns rather than propagating. Verify success by
    // waiting briefly for the TAS shared-memory section that
    // TAS_Helper.dll creates on init (`Local\SupremeTAS`); without it
    // we'd report "DLL injected" even when no real injection occurred,
    // and tas_ui would silently fail to connect.
    if !wait_for_shared_memory("Local\\SupremeTAS", Duration::from_secs(5)) {
        return Err(
            "Injector.exe exited successfully, but TAS shared memory was not created. \
             TAS_Helper.dll may have failed to attach (e.g., Supreme.exe is elevated, \
             or a dependency is missing)."
                .to_string(),
        );
    }

    // Launch tas_ui.exe (SSB Inspect) as a detached process
    let tas_ui_path = tas_folder.join("tas_ui.exe");
    if tas_ui_path.exists() {
        let _ = Command::new(&tas_ui_path)
            .current_dir(&tas_folder)
            .creation_flags(0x00000008) // DETACHED_PROCESS
            .spawn();
    }

    Ok("TAS_Helper.dll injected".to_string())
}

/// Poll for a Windows named shared-memory section by attempting to open it
/// with `OpenFileMappingW`. Returns true as soon as the mapping exists,
/// false on timeout. Used to verify TAS_Helper.dll actually attached and
/// initialised, rather than just trusting Injector.exe's exit code.
fn wait_for_shared_memory(name: &str, timeout: Duration) -> bool {
    use std::ffi::c_void;
    use std::iter;

    // Win32's canonical type names, kept verbatim so the FFI signatures read
    // like the SDK headers they mirror. Clippy 1.97 (the CI toolchain) flags
    // fully-capitalized acronyms under -D warnings; renaming these to Dword
    // etc. would be strictly less legible.
    #[allow(clippy::upper_case_acronyms)]
    type HANDLE = *mut c_void;
    #[allow(clippy::upper_case_acronyms)]
    type DWORD = u32;
    #[allow(clippy::upper_case_acronyms)]
    type BOOL = i32;
    const FILE_MAP_READ: DWORD = 0x0004;

    unsafe extern "system" {
        fn OpenFileMappingW(access: DWORD, inherit: BOOL, name: *const u16) -> HANDLE;
        fn CloseHandle(h: HANDLE) -> BOOL;
    }

    let wide: Vec<u16> = name.encode_utf16().chain(iter::once(0)).collect();
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        unsafe {
            let h = OpenFileMappingW(FILE_MAP_READ, 0, wide.as_ptr());
            if !h.is_null() {
                CloseHandle(h);
                return true;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

fn wait_for_finished_log(log_path: &PathBuf) -> Result<String, String> {
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(5);

    while start_time.elapsed() < timeout_duration {
        if let Ok(log_contents) = std::fs::read_to_string(log_path)
            && log_contents.contains("Finished.")
        {
            return Ok("Finished".to_string());
        }
        // Ignore error case and continue waiting
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let formatted_log_path = log_path.display().to_string().replace("\\", "/");
    Err(format!(
        "Timeout waiting for 'Finished.' in log {formatted_log_path}"
    ))
}
