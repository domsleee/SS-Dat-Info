//! The take in the shared recording buffer: who rode it, under which
//! physics, on which track, in which input model, and how far its recorded
//! trajectory is still what its input produces. The input and positions stay
//! in the mapping; every change to them that the UI makes goes through here,
//! so the take and its identity are replaced together.

use tas_shared::TasSharedState;

use crate::panels::input_script::{self, InputEvent};
use crate::recording::{self, IdentityStamps, RecordingHistory, RecordingSnapshot};
use crate::ui_log::UiLog;

/// What the take in the buffer is. Each half is `None` when unknown.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TakeIdentity {
    /// Physics-mode stamp (renderer + x87 precision): shown next to the live
    /// mode; a mismatch means the buffer's inputs were recorded under
    /// different rounding.
    pub(crate) physics: Option<String>,
    /// Rider stamp (character · stance).
    pub(crate) rider: Option<String>,
    /// Raw identity words. Save paths stamp the file with these, never the
    /// live game — `None` halves stay unknown.
    pub(crate) stamps: Option<IdentityStamps>,
    /// Track code (e.g. "FE"). PLAY/CONT refuse a take from another track:
    /// its spawn is elsewhere, so it can never match.
    pub(crate) level: Option<String>,
}

impl TakeIdentity {
    /// A `.tasrec` file's header; the track comes from its `<CODE>-` name.
    pub(crate) fn from_file(meta: &recording::RecordingMetadata, path: &std::path::Path) -> Self {
        Self {
            physics: meta.physics_label(),
            rider: meta.rider_label(),
            stamps: Some(IdentityStamps::from_metadata(meta)),
            level: tas_shared::level::code_from_recording_name(&path.to_string_lossy())
                .map(Into::into),
        }
    }

    /// The selected history entry's stamps.
    pub(crate) fn of_current_entry(history: &RecordingHistory) -> Option<Self> {
        let entry = history
            .current_index()
            .and_then(|i| history.entries().get(i))?;
        Some(Self {
            physics: entry.physics.clone(),
            rider: entry.rider.clone(),
            stamps: Some(entry.stamps.clone()),
            level: entry.level.clone(),
        })
    }
}

#[derive(Default)]
pub(crate) struct TakeBuffer {
    pub(crate) identity: TakeIdentity,
}

impl TakeBuffer {
    pub(crate) fn stamps(&self) -> Option<&IdentityStamps> {
        self.identity.stamps.as_ref()
    }

    /// The watcher's limit for this take: u32::MAX = the whole trajectory.
    pub(crate) fn trajectory_limit(&self) -> u32 {
        self.stamps()
            .map_or(u32::MAX, IdentityStamps::trajectory_limit)
    }

    /// The take was recorded in this session, or its buffer now belongs to
    /// another game: its stamps are these.
    pub(crate) fn replace_identity(&mut self, identity: TakeIdentity) {
        self.identity = identity;
    }

    /// A take recorded live: by definition in the live physics mode, as the
    /// live character / stance, on the live track.
    pub(crate) fn recorded_live(
        &mut self,
        history: &RecordingHistory,
        state: Option<&TasSharedState>,
        level: Option<&str>,
    ) {
        self.identity = TakeIdentity {
            physics: history.live_physics().map(str::to_string),
            rider: history.live_rider().map(str::to_string),
            stamps: state.map(IdentityStamps::from_live),
            level: level.map(Into::into),
        };
    }

    /// Load a `.tasrec` into the buffer, stamps and all. False when it did
    /// not load (logged); the buffer may then be partly written.
    pub(crate) fn load_file(
        &mut self,
        state: &mut TasSharedState,
        log: &mut UiLog,
        path: &std::path::Path,
    ) -> bool {
        // load_recording_path logs a mismatch with the live game itself.
        let Some(meta) = recording::load_recording_path(state, log, path) else {
            return false;
        };
        self.identity = TakeIdentity::from_file(&meta, path);
        true
    }

    /// Restore the history entry `select` picks, with that entry's stamps.
    /// Returns its tick count; None when there was nothing to restore.
    pub(crate) fn restore_from(
        &mut self,
        state: Option<&mut TasSharedState>,
        history: &mut RecordingHistory,
        select: impl FnOnce(&mut RecordingHistory) -> Option<&RecordingSnapshot>,
    ) -> Option<u32> {
        let count = {
            let snapshot = select(history)?;
            if let Some(state) = state {
                snapshot.restore_to(state);
            }
            snapshot.recorded_count
        };
        self.adopt_current_entry(history);
        Some(count)
    }

    /// Take the selected history entry's stamps (it holds the take in the
    /// buffer). False when there is no selected entry.
    pub(crate) fn adopt_current_entry(&mut self, history: &RecordingHistory) -> bool {
        match TakeIdentity::of_current_entry(history) {
            Some(identity) => {
                self.identity = identity;
                true
            }
            None => false,
        }
    }

    /// Warn when the take's physics mode or rider differs from the live
    /// game's (24-bit DirectX vs 53-bit OpenGL round the sim differently).
    pub(crate) fn warn_live_mismatch(&self, log: &mut UiLog, history: &RecordingHistory) {
        recording::warn_identity_mismatch(
            log,
            (self.identity.physics.as_deref(), history.live_physics()),
            (self.identity.rider.as_deref(), history.live_rider()),
        );
    }

    /// The model of the take in the buffer: the loaded take's stamp, or,
    /// with nothing loaded in this session (a UI opened on a buffer the DLL
    /// kept), the model the DLL holds for it.
    pub(crate) fn input_model(&self, state: Option<&TasSharedState>) -> u32 {
        match self.stamps() {
            Some(identity) => identity.input_model_or_injected(),
            None => state.map_or(tas_shared::TAS_INPUT_MODEL_INJECTED, |s| s.input_model),
        }
    }

    /// The track of the take in the buffer: its stamp, else its spawn
    /// position when that is unambiguous. None = unknown.
    pub(crate) fn level(&self, state: Option<&TasSharedState>) -> Option<String> {
        self.identity.level.clone().or_else(|| {
            let state = state?;
            (state.recorded_count > 0)
                .then(|| crate::start_line::level_code_from_spawn(&state.rec_coords[0]))
                .flatten()
                .map(Into::into)
        })
    }

    /// Why the take in the buffer cannot replay in the live game, when both
    /// sides are known: another track, rider or x87 precision.
    pub(crate) fn replay_mismatch(
        &self,
        state: &TasSharedState,
        live_rider: Option<&str>,
    ) -> Option<String> {
        let live_level =
            tas_shared::resolved_level_id(state).and_then(tas_shared::level::code_from_id);
        if let (Some(take), Some(live)) = (self.level(Some(state)), live_level) {
            if take != live {
                return Some(format!(
                    "this take was recorded on {take} but the game is on {live}. Restore or \
                     load a {live} take, or go back to {take}."
                ));
            }
        }
        // Character and stance are set when the level is entered from the
        // menu, so the advice names the screen; a restart does not change them.
        if let Some(advice) =
            tas_shared::rider_mismatch_advice(self.identity.rider.as_deref(), live_rider)
        {
            return Some(advice);
        }
        let take_bits = self
            .stamps()
            .and_then(|i| i.fpu_control_word)
            .map(tas_shared::fpu_precision_bits)
            .filter(|&bits| bits != 0);
        let live_bits =
            Some(tas_shared::fpu_precision_bits(state.fpu_control_word)).filter(|&bits| bits != 0);
        if let (Some(take), Some(live)) = (take_bits, live_bits) {
            if take != live {
                return Some(format!(
                    "this take was recorded at {take}-bit x87 precision but the game runs at \
                     {live}-bit: the physics round differently. Pick the take's renderer in \
                     Display_Config and relaunch."
                ));
            }
        }
        None
    }

    /// Write an edit's full event list over the stopped take's input.
    /// Past the first changed tick the recorded trajectory is stale, so the
    /// take's watcher must stop there.
    pub(crate) fn apply_edit(&mut self, state: &mut TasSharedState, events: &[InputEvent]) {
        let total = state.recorded_count;
        let len = (total as usize).min(state.input_log.len());
        let before = state.input_log[..len].to_vec();
        input_script::apply_events_to_log(&mut state.input_log, total, events);
        let first_changed = before
            .iter()
            .zip(state.input_log[..len].iter())
            .position(|(a, b)| a != b);
        if let Some(tick) = first_changed {
            let tick = tick as u32;
            let model = state.input_model;
            let identity = self.identity.stamps.get_or_insert_with(|| IdentityStamps {
                input_model: Some(model),
                ..Default::default()
            });
            identity.trajectory_ticks = Some(identity.trajectory_limit().min(tick));
        }
    }

    /// At a CONT splice, take the replayed prefix as the new take's
    /// trajectory. Past an input edit the recorded one is stale, and the
    /// replay is what the take's input produces; before it they are equal.
    pub(crate) fn adopt_replayed_prefix(state: &mut TasSharedState, splice: u32) {
        let rec_gate = recording_gate(state);
        let live_gate = state.gate_index;
        if rec_gate == 0 || live_gate == 0 {
            return;
        }
        let (dst, src) = (rec_gate as usize, live_gate as usize);
        let n = (splice.saturating_sub(rec_gate) as usize)
            .min(state.rec_coords.len().saturating_sub(dst))
            .min(state.play_coords.len().saturating_sub(src));
        state.rec_coords[dst..dst + n].copy_from_slice(&state.play_coords[src..src + n]);
    }
}

/// The take's gate (its first-moving frame), or 0 if it never moves. Uses
/// the shared `detect_first_moving` so the app and the harness align
/// identically.
pub(crate) fn recording_gate(state: &TasSharedState) -> u32 {
    tas_shared::align::detect_first_moving(&state.rec_coords, state.recorded_count).unwrap_or(0)
}
