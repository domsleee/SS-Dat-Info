//! Recording data model: session labels and finish times, plus the submodules for
//! the `.tasrec` format, snapshots, history, crash recovery and file dialogs.

mod dialogs;
mod file;
mod history;
mod recovery;
mod snapshot;
#[cfg(test)]
mod test_support;

pub use dialogs::{load_recording_path, pick_recording_path, save_dialog_with_segments};
pub use file::{IdentityStamps, RecordingFile, Segment, SegmentTracker};
pub use history::{HistoryEntry, HistoryEntryKind, RecordingHistory};
pub use recovery::{RecoverySessionContext, RecoveryStore, RecoveryWriter};
pub use snapshot::{PersistedSnapshot, RecordingSnapshot};

use serde::{Deserialize, Serialize};

const TAS_TICKS_PER_SECOND: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingSessionKind {
    Rec,
    Continue,
}

/// Format TAS tick durations as clock time (`m:ss.cc` or `h:mm:ss.cc`) at 100 Hz.
pub fn format_recording_duration(ticks: u32) -> String {
    let total_seconds = ticks / TAS_TICKS_PER_SECOND;
    let centiseconds = ticks % TAS_TICKS_PER_SECOND;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes >= 60 {
        let hours = minutes / 60;
        let rem_minutes = minutes % 60;
        format!(
            "{}:{:02}:{:02}.{:02}",
            hours, rem_minutes, seconds, centiseconds
        )
    } else {
        format!("{}:{:02}.{:02}", minutes, seconds, centiseconds)
    }
}

pub fn completed_session_label(
    kind: RecordingSessionKind,
    start_tick: u32,
    end_tick: u32,
) -> Option<String> {
    if end_tick <= start_tick {
        return None;
    }

    let segment_ticks = end_tick - start_tick;
    let segment = format_recording_duration(segment_ticks);

    let label = match kind {
        RecordingSessionKind::Rec => format!("Recorded {}", segment),
        RecordingSessionKind::Continue => format!(
            "Continued from {}, total {}",
            format_recording_duration(start_tick),
            format_recording_duration(end_tick)
        ),
    };
    Some(label)
}

/// The race time of a session that ended at the finish line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishStamp {
    /// Centiseconds (= game ticks at 100 Hz).
    pub cs: u32,
    /// `true` = the HUD timer's value; `false` = derived from the recording's
    /// start-line / finish-line crossings (within ~0.05 s of the game's timer).
    pub exact: bool,
}

/// Display form of a finish time: "3:50.57" when exact, "~3:50.62" when
/// geometry-derived, so the label never overstates its precision.
pub fn format_finish_time(cs: u32, exact: bool) -> String {
    if exact {
        format_recording_duration(cs)
    } else {
        format!("~{}", format_recording_duration(cs))
    }
}

/// Label for a session that ended by crossing the finish line, so the entry
/// reads "Finish 0:53.34" rather than by its length.
pub fn finished_session_label(finish: FinishStamp) -> String {
    format!("Finish {}", format_finish_time(finish.cs, finish.exact))
}

/// Race time (centiseconds) of a finished run from the recording alone, for
/// when the DLL's HUD race-timer feed is empty: the in-game timer runs from
/// the START-LINE trigger to the finish line, so use the geometric start
/// crossing when the track has one; a recording that never crossed a start
/// line (unknown track) falls back to first movement, then to tick 0.
pub fn geometry_race_time_cs(
    finish_tick: u32,
    start_cross_tick: Option<u32>,
    first_moving: Option<u32>,
) -> u32 {
    let start = start_cross_tick.or(first_moving).unwrap_or(0);
    finish_tick.saturating_sub(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_recording_duration_uses_clock_format() {
        assert_eq!(format_recording_duration(0), "0:00.00");
        assert_eq!(format_recording_duration(2303), "0:23.03");
        assert_eq!(format_recording_duration(5303), "0:53.03");
        assert_eq!(format_recording_duration(65536), "10:55.36");
        assert_eq!(format_recording_duration(372300), "1:02:03.00");
    }

    #[test]
    fn completed_session_label_formats_rec_and_continue() {
        assert_eq!(
            completed_session_label(RecordingSessionKind::Rec, 0, 2303).as_deref(),
            Some("Recorded 0:23.03")
        );
        assert_eq!(
            completed_session_label(RecordingSessionKind::Continue, 3000, 5303).as_deref(),
            Some("Continued from 0:30.00, total 0:53.03")
        );
        assert!(
            completed_session_label(RecordingSessionKind::Rec, 100, 100).is_none(),
            "zero-length sessions should not create labels"
        );
    }

    #[test]
    fn finished_session_label_is_the_race_time() {
        let exact = |cs| FinishStamp { cs, exact: true };
        let approx = |cs| FinishStamp { cs, exact: false };
        assert_eq!(finished_session_label(exact(5334)), "Finish 0:53.34");
        assert_eq!(finished_session_label(exact(23_546)), "Finish 3:55.46");
        // Geometry-derived times are marked: the planes sit ~5 cs from the
        // engine's triggers (3:50.62 measured vs the game's 3:50.57).
        assert_eq!(finished_session_label(approx(23_062)), "Finish ~3:50.62");
        assert_eq!(format_finish_time(23_057, true), "3:50.57");
        assert_eq!(format_finish_time(23_062, false), "~3:50.62");
    }

    #[test]
    fn geometry_race_time_runs_from_the_start_line_not_first_movement() {
        // Measured Forest Easy run: first moved at 288, crossed the start line
        // at ~789, finish watch fired at 23846; the game said 3:50.57.
        assert_eq!(geometry_race_time_cs(23_846, Some(789), Some(288)), 23_057);
        // No start line known for the track: first movement is the best guess.
        assert_eq!(geometry_race_time_cs(23_846, None, Some(288)), 23_558);
        assert_eq!(geometry_race_time_cs(100, None, None), 100);
        assert_eq!(
            geometry_race_time_cs(50, Some(80), None),
            0,
            "never negative"
        );
    }

    #[test]
    fn format_recording_duration_boundary_values() {
        // 0 ticks
        assert_eq!(format_recording_duration(0), "0:00.00");
        // 1 tick = 0.01s
        assert_eq!(format_recording_duration(1), "0:00.01");
        // 99 ticks = 0.99s
        assert_eq!(format_recording_duration(99), "0:00.99");
        // 100 ticks = 1.00s
        assert_eq!(format_recording_duration(100), "0:01.00");
        // 5999 ticks = 59.99s (just under 1 minute)
        assert_eq!(format_recording_duration(5999), "0:59.99");
        // 6000 ticks = 1:00.00
        assert_eq!(format_recording_duration(6000), "1:00.00");
        // 359999 ticks = 59:59.99 (just under 1 hour)
        assert_eq!(format_recording_duration(359999), "59:59.99");
        // 360000 ticks = 1:00:00.00
        assert_eq!(format_recording_duration(360000), "1:00:00.00");
        // 65536 ticks: past the 16-bit range
        assert_eq!(format_recording_duration(65536), "10:55.36");
    }

    #[test]
    fn completed_session_label_zero_length_is_none() {
        assert!(completed_session_label(RecordingSessionKind::Rec, 50, 50).is_none());
        assert!(completed_session_label(RecordingSessionKind::Rec, 50, 49).is_none());
        assert!(completed_session_label(RecordingSessionKind::Continue, 100, 100).is_none());
    }

    #[test]
    fn completed_session_label_continue_shows_start_and_total() {
        let label = completed_session_label(RecordingSessionKind::Continue, 6000, 12000).unwrap();
        assert_eq!(label, "Continued from 1:00.00, total 2:00.00");
    }
}
