use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::panels::input_script::{parse_script, InputEvent};

pub struct ScriptWatch {
    path: PathBuf,
    observed: String,
    changed_at: Option<Instant>,
}

impl ScriptWatch {
    pub fn fresh_path() -> PathBuf {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "ssb_inputs_{}_{}_{}.tas",
            std::process::id(),
            stamp,
            id
        ))
    }

    pub fn new(path: PathBuf, text: String) -> Self {
        Self {
            path,
            observed: text,
            changed_at: None,
        }
    }

    pub fn poll(&mut self) -> Option<Result<Vec<InputEvent>, String>> {
        let text = std::fs::read_to_string(&self.path).ok()?;
        self.observe(text, Instant::now())
    }

    fn observe(&mut self, text: String, now: Instant) -> Option<Result<Vec<InputEvent>, String>> {
        if text != self.observed {
            self.observed = text;
            self.changed_at = Some(now);
            return None;
        }
        if now.duration_since(self.changed_at?) < Duration::from_millis(500) {
            return None;
        }
        self.changed_at = None;
        let (events, errors) = parse_script(&self.observed);
        if let Some(error) = errors.first() {
            return Some(Err(format!(
                "line {}: {} (inputs unchanged)",
                error.line, error.reason
            )));
        }
        if events.is_empty()
            && !self
                .observed
                .lines()
                .any(|line| line.trim() == "# clear inputs")
        {
            return Some(Err(
                "empty script ignored; use # clear inputs to intentionally clear all inputs".into(),
            ));
        }
        Some(Ok(events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_and_invalid_saves_do_not_replace_inputs() {
        let mut watch = ScriptWatch::new(PathBuf::new(), "1-5 press left".into());
        let now = Instant::now();
        assert!(watch.observe(String::new(), now).is_none());
        assert!(watch
            .observe(String::new(), now + Duration::from_secs(1))
            .unwrap()
            .is_err());
        assert!(watch.observe("1-5 press left\n6-".into(), now).is_none());
        assert!(watch
            .observe("1-5 press left\n6-".into(), now + Duration::from_secs(1))
            .unwrap()
            .is_err());
        assert!(watch.observe("2-6 press right".into(), now).is_none());
        assert!(watch
            .observe("2-6 press right".into(), now + Duration::from_millis(499))
            .is_none());
        let events = watch
            .observe("2-6 press right".into(), now + Duration::from_secs(1))
            .unwrap()
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].start, 2);
        assert!(watch
            .observe("2-6 press right".into(), now + Duration::from_secs(2))
            .is_none());
    }

    #[test]
    fn every_write_restarts_stability_window_and_clear_is_explicit() {
        let mut watch = ScriptWatch::new(PathBuf::new(), String::new());
        let now = Instant::now();
        watch.observe("1-5 press left".into(), now);
        watch.observe("# clear inputs".into(), now + Duration::from_millis(400));
        assert!(watch
            .observe("# clear inputs".into(), now + Duration::from_millis(600))
            .is_none());
        assert!(watch
            .observe("# clear inputs".into(), now + Duration::from_secs(1))
            .unwrap()
            .unwrap()
            .is_empty());
    }
}
