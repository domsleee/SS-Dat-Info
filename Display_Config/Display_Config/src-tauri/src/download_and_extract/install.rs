use anyhow::{Context, Result, bail, ensure};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::Cursor;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, TryLockError};

pub(super) static INSTALLATION_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn when_not_installing(action: impl FnOnce()) {
    let _guard = match INSTALLATION_LOCK.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::Poisoned(error)) => error.into_inner(),
        Err(TryLockError::WouldBlock) => return,
    };
    action();
}

fn lock_folder(root: &Path) -> Result<File> {
    // The OS releases this lock on exit; the empty file can be reused.
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .share_mode(0)
        .open(root.join(".display-config-update.lock"))
        .context(
            "Cannot lock the game folder for updating (not writable, or another update is running)",
        )
}

pub(super) fn cleanup(root: &Path) {
    if let Ok(_lock) = lock_folder(root) {
        cleanup_staging(root);
    }
}

const REQUIRED_FILES: [&str; 3] = [
    "Display_Config.exe",
    "Display_Config_Resources/Display_Config_Helper.dll",
    "Display_Config_Resources/Injector.exe",
];

struct Staging {
    path: PathBuf,
    keep: bool,
}

impl Drop for Staging {
    fn drop(&mut self) {
        if !self.keep
            && let Err(error) = remove_staging(&self.path)
        {
            // Windows can keep the old executable locked until this process exits.
            log::warn!("Update files retained at {}: {error}", self.path.display());
        }
    }
}

fn remove_staging(path: &Path) -> std::io::Result<()> {
    // Keep the completion marker if the running executable cannot be deleted yet.
    match fs::remove_dir_all(path.join("backup")) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    fs::remove_dir_all(path)
}

fn cleanup_staging(root: &Path) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with(".display-config-update-")
                && entry
                    .file_type()
                    .is_ok_and(|kind| kind.is_dir() && !kind.is_symlink())
                && (entry.path().join("complete").is_file()
                    || matches!(fs::symlink_metadata(entry.path().join("backup")),
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound))
            {
                let _ = remove_staging(&entry.path());
            }
        }
    }
}

pub(super) fn install_zip(
    bytes: Vec<u8>,
    root: &Path,
    cancelled: impl Fn() -> bool,
    before_install: impl FnOnce() -> Result<bool>,
) -> Result<bool> {
    let _lock = lock_folder(root)?;
    cleanup_staging(root);
    let staging_path = root.join(format!(".display-config-update-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&staging_path)?;
    let mut staging = Staging {
        path: staging_path,
        keep: false,
    };
    let new = staging.path.join("new");
    let backup = staging.path.join("backup");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let mut paths = Vec::new();
    let mut names = HashSet::new();
    for index in 0..archive.len() {
        if cancelled() {
            return Ok(false);
        }
        let mut entry = archive.by_index(index)?;
        let name = entry.name().replace('\\', "/");
        let name = name.trim_end_matches('/');
        ensure!(
            !entry.is_symlink(),
            "Update contains a symbolic link: {name}"
        );
        ensure!(
            name.split('/').all(|part| !part.is_empty()
                && part != "."
                && part != ".."
                && !part.ends_with(['.', ' '])
                && !part.contains([':', '<', '>', '"', '|', '?', '*'])),
            "Invalid update path: {name}"
        );
        ensure!(
            name.eq_ignore_ascii_case("Display_Config.exe")
                || name.eq_ignore_ascii_case("Display_Config_Resources")
                || name
                    .to_ascii_lowercase()
                    .starts_with("display_config_resources/"),
            "Unexpected update path: {name}"
        );
        if entry.is_dir() {
            continue;
        }
        ensure!(
            names.insert(name.to_ascii_lowercase()),
            "Duplicate update file: {name}"
        );
        if REQUIRED_FILES
            .iter()
            .any(|required| name.eq_ignore_ascii_case(required))
        {
            ensure!(entry.size() > 0, "Empty update binary: {name}");
        }
        let path = PathBuf::from(name);
        let destination = new.join(&path);
        fs::create_dir_all(destination.parent().unwrap())?;
        let mut file = File::create(&destination)?;
        std::io::copy(&mut entry, &mut file).with_context(|| format!("Cannot extract {name}"))?;
        paths.push(path);
    }
    for required in REQUIRED_FILES {
        ensure!(
            names.contains(&required.to_ascii_lowercase()),
            "Update is missing {required}"
        );
    }
    for path in &paths {
        let target = root.join(path);
        ensure!(
            !target.try_exists()? || target.is_file(),
            "Update destination is not a file: {}",
            target.display()
        );
        let mut destination = root.to_path_buf();
        for component in path.components() {
            destination.push(component);
            if let Ok(metadata) = fs::symlink_metadata(&destination) {
                use std::os::windows::fs::MetadataExt;
                ensure!(
                    metadata.file_attributes() & 0x400 == 0,
                    "Update destination is a reparse point: {}",
                    destination.display()
                );
            }
        }
    }
    let _installing = INSTALLATION_LOCK
        .lock()
        .map_err(|_| anyhow::anyhow!("Installation lock poisoned"))?;
    if cancelled() || !before_install()? {
        return Ok(false);
    }

    let mut replaced = Vec::new();
    let result = (|| -> Result<()> {
        for path in &paths {
            let destination = root.join(path);
            let old = backup.join(path);
            fs::create_dir_all(destination.parent().unwrap())?;
            let existed = destination.try_exists()?;
            if existed {
                ensure!(
                    destination.is_file(),
                    "Update destination is not a file: {}",
                    destination.display()
                );
                fs::create_dir_all(old.parent().unwrap())?;
                fs::rename(&destination, &old)
                    .with_context(|| format!("Cannot back up {}", destination.display()))?;
            }
            replaced.push((path, existed));
            fs::rename(new.join(path), &destination)
                .with_context(|| format!("Cannot install {}", destination.display()))?;
        }
        Ok(())
    })();
    if let Err(error) = result {
        let mut failures = Vec::new();
        for (path, existed) in replaced.into_iter().rev() {
            let destination = root.join(path);
            let restored = (|| -> std::io::Result<()> {
                match fs::remove_file(&destination) {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e),
                }
                if existed {
                    fs::rename(backup.join(path), &destination)?;
                }
                Ok(())
            })();
            if let Err(e) = restored {
                failures.push(format!("{}: {e}", path.display()));
            }
        }
        if !failures.is_empty() {
            staging.keep = true;
            bail!(
                "{error:#}. Could not restore {}. Backups retained at {}",
                failures.join(", "),
                backup.display()
            );
        }
        return Err(error.context("Update failed; previous files were restored"));
    }
    if let Err(error) = fs::write(staging.path.join("complete"), []) {
        log::warn!(
            "Could not mark completed update at {}: {error}",
            staging.path.display()
        );
    }
    Ok(true)
}

#[cfg(test)]
mod tests;
