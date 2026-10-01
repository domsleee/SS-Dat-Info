//! Editing the take's input: the timeline's gesture state, the edit a
//! gesture or the external script produced, and that script's watcher. An
//! edit applies only while stopped; the app applies it (`TakeBuffer`).

use crate::panels::input_script::InputEvent;
use crate::panels::timeline::TimelineEdit;
use crate::script_watch::ScriptWatch;
use crate::ui_log::UiLog;

/// The take's full new event list, from one finished gesture or script save.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Edit {
    pub(crate) events: Vec<InputEvent>,
    /// Push one undo snapshot for it.
    pub(crate) commit: bool,
    /// The history label; empty = "Edited inputs".
    pub(crate) label: String,
}

/// An edit produced by the timeline or the script, applied to `input_log`
/// at the start of the next frame.
#[derive(Debug, Default, PartialEq)]
pub(crate) enum PendingEdit {
    #[default]
    None,
    Ready(Edit),
    /// It arrived while a run was active, so one auto-STOP went out; it
    /// applies once the mode is Off. Keeps STOP (and its log line) from
    /// being re-sent every frame while the game settles.
    WaitingForStop(Edit),
}

#[derive(Default)]
pub(crate) struct Editor {
    pub(crate) timeline: TimelineEdit,
    pub(crate) pending: PendingEdit,
    /// Stable-content watcher for this recording's external script.
    pub(crate) script_watch: Option<ScriptWatch>,
}

impl Editor {
    /// Discard the editors and the queued gesture: their recording is being
    /// replaced.
    pub(crate) fn detach(&mut self) {
        self.script_watch = None;
        self.pending = PendingEdit::None;
        self.timeline = TimelineEdit::default();
    }

    pub(crate) fn has_pending(&self) -> bool {
        self.pending != PendingEdit::None
    }

    /// Queue `edit` in place of any queued one. A STOP already sent for the
    /// queued one serves this one too.
    pub(crate) fn queue(&mut self, edit: Edit) {
        self.pending = match self.pending {
            PendingEdit::WaitingForStop(_) => PendingEdit::WaitingForStop(edit),
            _ => PendingEdit::Ready(edit),
        };
    }

    /// The queued edit must wait for a run to stop. True the first time, when
    /// the caller sends the STOP.
    pub(crate) fn wait_for_stop(&mut self) -> bool {
        match std::mem::take(&mut self.pending) {
            PendingEdit::Ready(edit) => {
                self.pending = PendingEdit::WaitingForStop(edit);
                true
            }
            other => {
                self.pending = other;
                false
            }
        }
    }

    /// The queued edit, to apply now.
    pub(crate) fn take_pending(&mut self) -> Option<Edit> {
        match std::mem::take(&mut self.pending) {
            PendingEdit::None => None,
            PendingEdit::Ready(edit) | PendingEdit::WaitingForStop(edit) => Some(edit),
        }
    }

    /// Start watching the external script just written to `path`.
    pub(crate) fn watch_script(&mut self, path: std::path::PathBuf, script: String) {
        self.script_watch = Some(ScriptWatch::new(path, script));
    }

    /// Poll the externally-edited `.tas` file; on save, parse it and queue
    /// the inputs for application (reload-on-save). No-op until a file is
    /// opened via "Open in external editor".
    pub(crate) fn poll_script(&mut self, log: &mut UiLog) {
        let Some(result) = self.script_watch.as_mut().and_then(|watch| watch.poll()) else {
            return;
        };
        match result {
            Ok(events) => {
                let n = events.len();
                self.queue(Edit {
                    events,
                    commit: true,
                    label: "Loaded inputs from script".to_string(),
                });
                log.push(format!("[script] reloaded {} inputs", n));
            }
            Err(e) => log.push(format!("[script] reload failed: {e}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit(label: &str) -> Edit {
        Edit {
            events: Vec::new(),
            commit: true,
            label: label.into(),
        }
    }

    /// One STOP per run: an edit queued after it went out waits for it.
    #[test]
    fn a_newer_edit_waits_for_the_stop_already_sent() {
        let mut editor = Editor::default();
        editor.queue(edit("a"));
        assert!(editor.wait_for_stop());
        editor.queue(edit("b"));
        assert!(!editor.wait_for_stop(), "no second STOP");
        assert_eq!(editor.take_pending(), Some(edit("b")));
        assert!(!editor.has_pending());
    }
}
