use super::*;
use std::cell::Cell;
use std::io::Write;

struct Game(PathBuf);

impl Game {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("ss-install-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&root).unwrap();
        for path in REQUIRED_FILES {
            let file = root.join(path);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(file, b"original").unwrap();
        }
        Self(root)
    }

    fn assert_original(&self) {
        for path in REQUIRED_FILES {
            assert_eq!(fs::read(self.0.join(path)).unwrap(), b"original");
        }
    }

    fn assert_clean(&self) {
        assert!(!fs::read_dir(&self.0).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".display-config-update-")
        }));
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn archive(paths: &[&str]) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for path in paths {
        zip.start_file(
            *path,
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored),
        )
        .unwrap();
        zip.write_all(b"updated binary").unwrap();
    }
    zip.finish().unwrap().into_inner()
}

#[test]
fn installs_complete_update_and_preserves_settings() {
    let game = Game::new();
    let settings = game
        .0
        .join("Display_Config_Resources/Display_Config_Helper.json");
    fs::write(&settings, b"settings").unwrap();
    assert!(install_zip(archive(&REQUIRED_FILES), &game.0, || false, || Ok(true)).unwrap());
    for path in REQUIRED_FILES {
        assert_eq!(fs::read(game.0.join(path)).unwrap(), b"updated binary");
    }
    assert_eq!(fs::read(settings).unwrap(), b"settings");
    game.assert_clean();
}

#[test]
fn locked_dll_rolls_back_replaced_and_new_files() {
    let game = Game::new();
    let _locked = OpenOptions::new()
        .read(true)
        .share_mode(1)
        .open(game.0.join(REQUIRED_FILES[1]))
        .unwrap();
    let error = install_zip(
        archive(&[
            REQUIRED_FILES[0],
            "Display_Config_Resources/new.dll",
            REQUIRED_FILES[1],
            REQUIRED_FILES[2],
        ]),
        &game.0,
        || false,
        || Ok(true),
    )
    .unwrap_err();
    assert!(format!("{error:#}").contains("previous files were restored"));
    game.assert_original();
    assert!(!game.0.join("Display_Config_Resources/new.dll").exists());
    game.assert_clean();
}

#[test]
fn rejects_incomplete_unsafe_and_case_duplicate_archives_before_installing() {
    for paths in [
        vec![REQUIRED_FILES[0]],
        vec![
            REQUIRED_FILES[0],
            REQUIRED_FILES[1],
            REQUIRED_FILES[2],
            "../outside.exe",
        ],
        vec![
            REQUIRED_FILES[0],
            REQUIRED_FILES[1],
            REQUIRED_FILES[2],
            "display_config.EXE",
        ],
        vec![
            REQUIRED_FILES[0],
            REQUIRED_FILES[1],
            REQUIRED_FILES[2],
            "Display_Config_Resources/../escape",
        ],
    ] {
        let game = Game::new();
        assert!(
            install_zip(
                archive(&paths),
                &game.0,
                || false,
                || panic!("must not install")
            )
            .is_err()
        );
        game.assert_original();
        game.assert_clean();
    }
}

#[test]
fn corrupt_zip_leaves_installed_files_untouched() {
    let game = Game::new();
    let mut bytes = archive(&REQUIRED_FILES);
    let offset = bytes
        .windows(14)
        .rposition(|bytes| bytes == b"updated binary")
        .unwrap();
    bytes[offset] ^= 1;
    assert!(install_zip(bytes, &game.0, || false, || panic!("must not install")).is_err());
    game.assert_original();
    game.assert_clean();
}

#[test]
fn cancellation_during_staging_and_at_install_boundary_preserves_originals() {
    for during_staging in [true, false] {
        let game = Game::new();
        let calls = Cell::new(0);
        assert!(
            !install_zip(
                archive(&REQUIRED_FILES),
                &game.0,
                || {
                    calls.set(calls.get() + 1);
                    during_staging && calls.get() == 2
                },
                || Ok(false)
            )
            .unwrap()
        );
        game.assert_original();
        game.assert_clean();
    }
}

#[test]
fn second_installer_cannot_run_while_folder_is_locked() {
    let game = Game::new();
    let _lock = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .share_mode(0)
        .open(game.0.join(".display-config-update.lock"))
        .unwrap();
    assert!(install_zip(archive(&REQUIRED_FILES), &game.0, || false, || Ok(true)).is_err());
    game.assert_original();
    game.assert_clean();
}

#[test]
fn terminated_process_leaves_a_reusable_lock_file() {
    use std::io::{BufRead, BufReader};
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};

    let game = Game::new();
    let lock_path = game.0.join(".display-config-update.lock");
    let mut child = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", r#"
            $ErrorActionPreference = 'Stop'
            $handle = [IO.File]::Open($env:SS_UPDATE_LOCK_TEST_PATH, 'OpenOrCreate', 'ReadWrite', 'None')
            [Console]::WriteLine('locked')
            [Threading.Thread]::Sleep(-1)
        "#])
        .env("SS_UPDATE_LOCK_TEST_PATH", &lock_path)
        .creation_flags(0x08000000)
        .stdout(Stdio::piped())
        .spawn().unwrap();
    let mut ready = String::new();
    let read = BufReader::new(child.stdout.take().unwrap()).read_line(&mut ready);
    // Terminate without running the owner's cleanup code.
    let _ = child.kill();
    child.wait().unwrap();
    read.unwrap();
    assert_eq!(ready.trim(), "locked");
    assert!(lock_path.exists());
    assert!(install_zip(archive(&REQUIRED_FILES), &game.0, || false, || Ok(true)).unwrap());
    game.assert_clean();
}

#[test]
fn can_replace_an_open_executable_that_allows_rename() {
    let game = Game::new();
    let _running = OpenOptions::new()
        .read(true)
        .share_mode(1 | 4)
        .open(game.0.join(REQUIRED_FILES[0]))
        .unwrap();
    assert!(install_zip(archive(&REQUIRED_FILES), &game.0, || false, || Ok(true)).unwrap());
    assert_eq!(
        fs::read(game.0.join(REQUIRED_FILES[0])).unwrap(),
        b"updated binary"
    );
}

#[test]
fn cleans_completed_backups_but_preserves_recovery_files() {
    let game = Game::new();
    let completed = game.0.join(".display-config-update-completed");
    let failed = game.0.join(".display-config-update-failed");
    for path in [&completed, &failed] {
        fs::create_dir_all(path.join("backup")).unwrap();
        fs::write(path.join("backup/Display_Config.exe"), b"backup").unwrap();
    }
    fs::write(completed.join("complete"), []).unwrap();
    let locked = OpenOptions::new()
        .read(true)
        .share_mode(1)
        .open(completed.join("backup/Display_Config.exe"))
        .unwrap();
    cleanup_completed(&game.0);
    assert!(completed.join("complete").exists());
    drop(locked);
    cleanup_completed(&game.0);
    assert!(!completed.exists());
    assert_eq!(
        fs::read(failed.join("backup/Display_Config.exe")).unwrap(),
        b"backup"
    );
}

#[test]
fn permits_empty_extra_resources_but_rejects_empty_required_binaries() {
    for empty_file in ["Display_Config_Resources/marker", REQUIRED_FILES[0]] {
        let game = Game::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for path in REQUIRED_FILES
            .into_iter()
            .chain(["Display_Config_Resources/marker"])
        {
            zip.start_file(path, zip::write::SimpleFileOptions::default())
                .unwrap();
            if path != empty_file {
                zip.write_all(b"updated binary").unwrap();
            }
        }
        let bytes = zip.finish().unwrap().into_inner();
        let result = install_zip(bytes, &game.0, || false, || Ok(true));
        if empty_file == REQUIRED_FILES[0] {
            assert!(result.is_err());
            game.assert_original();
        } else {
            assert!(result.unwrap());
            assert_eq!(fs::metadata(game.0.join(empty_file)).unwrap().len(), 0);
        }
        game.assert_clean();
    }
}
