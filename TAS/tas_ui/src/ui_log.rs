//! The session log: an in-memory ring the log panel renders (capped so a long
//! session cannot grow the UI without bound) mirrored line-for-line into
//! `tas_ui.log` under the data directory. `push` is the only writer, so the cap
//! applies to every line.

use std::io::Write;

const CAP: usize = 500;
/// Oldest lines dropped once the cap is hit.
const DRAIN: usize = 100;

#[derive(Default)]
pub struct UiLog {
    lines: Vec<String>,
    file: Option<std::fs::File>,
    /// Index up to which `lines` has been written to `file`.
    persisted: usize,
}

impl UiLog {
    /// `tas_ui.log` in append mode inside `dir`; a file that cannot be opened
    /// leaves the in-memory log as the only record rather than blocking startup.
    pub fn new(dir: &std::path::Path) -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("tas_ui.log"))
            .ok();
        Self {
            lines: Vec::new(),
            file,
            persisted: 0,
        }
    }

    pub fn push(&mut self, msg: impl AsRef<str>) {
        let ts = chrono::Local::now().format("%H:%M:%S");
        self.lines.push(format!("[{}] {}", ts, msg.as_ref()));
        if self.lines.len() > CAP {
            // The on-disk mirror already has these (flushed before any push
            // this frame); only the persisted cursor needs to follow.
            self.lines.drain(..DRAIN);
            self.persisted = self.persisted.saturating_sub(DRAIN);
        }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.persisted = 0;
    }

    /// Append the lines pushed since the last call to the file. Best effort:
    /// a write failure loses mirror lines, never the UI.
    pub fn flush_to_file(&mut self) {
        let Some(file) = self.file.as_mut() else {
            return;
        };
        if self.persisted >= self.lines.len() {
            return;
        }
        for line in &self.lines[self.persisted..] {
            let _ = writeln!(file, "{}", line);
        }
        let _ = file.flush();
        self.persisted = self.lines.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_applies_and_clear_resets_the_mirror_cursor() {
        let mut log = UiLog::default();
        for i in 0..(CAP + 1) {
            log.push(i.to_string());
        }
        assert_eq!(log.lines().len(), CAP + 1 - DRAIN);
        assert!(log.lines()[0].ends_with(&format!("] {}", DRAIN)));
        log.flush_to_file();
        log.clear();
        assert!(log.lines().is_empty());
        assert_eq!(log.persisted, 0);
    }
}
