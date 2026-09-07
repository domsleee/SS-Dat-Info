//! The wire protocol shared with TAS_Helper.dll: the constants, enums and
//! `repr(C)` structs that mirror `shared_state.hpp`, plus the seqlocked
//! readers over the live mapping.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::rider::TAS_CHARACTER_UNKNOWN;

pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";

/// Bumped whenever `TasSharedState` changes layout (mirrors shared_state.hpp).
pub const TAS_SHARED_VERSION: u32 = 51;
/// v46: size of the menu document buffer (JSON, NUL-terminated).
pub const TAS_MENU_DOC_MAX: usize = 4096;
/// v47: size of the menu command target (id or label, NUL-terminated).
pub const TAS_MENU_CMD_TARGET_MAX: usize = 64;
/// v47 menu command kinds (`menu_cmd_kind`).
pub const TAS_MENU_CMD_ACTIVATE: u32 = 1;
pub const TAS_MENU_CMD_FOCUS: u32 = 2;
pub const TAS_MENU_CMD_UP: u32 = 3;
pub const TAS_MENU_CMD_DOWN: u32 = 4;
pub const TAS_MENU_CMD_LEFT: u32 = 5;
pub const TAS_MENU_CMD_RIGHT: u32 = 6;
pub const TAS_MENU_CMD_TRIGGER: u32 = 7;
/// v47 menu command results (`menu_cmd_result`).
pub const TAS_MENU_RESULT_OK: u32 = 0;
pub const TAS_MENU_RESULT_NO_MENU: u32 = 1;
pub const TAS_MENU_RESULT_NOT_FOUND: u32 = 2;
pub const TAS_MENU_RESULT_DISABLED: u32 = 3;
pub const TAS_MENU_RESULT_BAD_KIND: u32 = 4;
pub const TAS_MENU_RESULT_FAULT: u32 = 5;
pub const TAS_MENU_RESULT_NOT_FOCUSABLE: u32 = 6;
pub const TAS_MENU_RESULT_STALE_PAGE: u32 = 7;
pub const TAS_MENU_RESULT_EXPIRED: u32 = 8;
pub const TAS_LEVEL_PATH_MAX: usize = 128;
pub const TAS_MENU_SCREEN_MAX: usize = 32;
pub const TAS_MAX_TICKS: usize = 65536;
pub const TAS_MAX_SEGMENTS: usize = 32;
pub const TAS_LOG_RING_SIZE: usize = 64;
pub const TAS_LOG_ENTRY_SIZE: usize = 120;

/// Value the DLL parks in the command slot while an out-of-cycle consumer
/// (level-scan worker / SwapBuffers hook) applies a STOP (`CAVE2_CMD_CLAIMED_STOP`
/// in cave2.hpp). It is not a `TasCommand`; readers treat it as "STOP in flight".
pub const TAS_CMD_CLAIMED_STOP: u32 = u32::MAX;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasCommand {
    Idle = 0,
    ArmRec = 1,
    ArmPlay = 2,
    Stop = 3,
    ArmContinue = 4,
    Restart = 5,
    /// Stop for an internal restart while retaining live-input protection.
    StopForRestart = 9,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasMode {
    Off = 0,
    Rec = 1,
    Play = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasLogSeverity {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

/// A single log ring entry (matches C++ TasLogEntry)
#[repr(C)]
pub struct TasLogEntry {
    pub sequence: u32,
    pub severity: u32,
    pub text: [u8; TAS_LOG_ENTRY_SIZE],
}

impl TasLogEntry {
    pub fn text_str(&self) -> &str {
        let len = self
            .text
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(TAS_LOG_ENTRY_SIZE);
        std::str::from_utf8(&self.text[..len]).unwrap_or("<invalid utf8>")
    }

    pub fn severity_enum(&self) -> TasLogSeverity {
        match self.severity {
            0 => TasLogSeverity::Debug,
            1 => TasLogSeverity::Info,
            2 => TasLogSeverity::Warn,
            3 => TasLogSeverity::Error,
            _ => TasLogSeverity::Info,
        }
    }
}

/// A segment boundary record (matches C++ TasSegmentBoundary, 8 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TasSegmentBoundary {
    pub frame: u32,            // Frame number where this segment starts
    pub input_log_offset: u32, // Offset into input_log for this segment
}

/// Input mask bit definitions (matches C++ TasInputBit)
pub mod input_bits {
    pub const LEFT: u8 = 0x01;
    pub const RIGHT: u8 = 0x02;
    pub const UP: u8 = 0x04;
    pub const DOWN: u8 = 0x08;
    pub const JUMP: u8 = 0x10;
    pub const SHIFT: u8 = 0x20;

    pub const ALL: &[(u8, &str, &str)] = &[
        (LEFT, "L", "Left"),
        (RIGHT, "R", "Right"),
        (UP, "U", "Up"),
        (DOWN, "D", "Down"),
        (JUMP, "J", "Jump"),
        (SHIFT, "S", "Shift"),
    ];
}

/// Must match the C++ TasSharedState layout exactly (TAS_Helper/src/shared_state.hpp).
/// All fields are naturally aligned (u32/f32 = 4 bytes, the perf counters 8),
/// so repr(C) suffices. Both sides pin the size and the group-boundary offsets
/// (see the layout tests) and bump `TAS_SHARED_VERSION` on any change.
#[repr(C)]
pub struct TasSharedState {
    pub version: u32,

    // Command region (UI writes, DLL reads)
    pub command: u32,
    /// 0 = natural ticks, N = force N ticks per frame (cave5).
    pub force_fixed_tick: u32,
    /// CMD_ARM_CONTINUE: splice point (PLAY 0..N, then REC).
    pub continue_from_frame: u32,

    // Status region (DLL writes, UI reads)
    pub mode: u32,
    pub recorded_count: u32,
    pub playback_pos: u32,
    pub player_x: f32,
    pub player_y: f32,
    pub player_z: f32,
    pub max_drift_x: f32,
    pub max_drift_z: f32,

    // Diagnostics
    pub bb3b10_call_count: u32,
    pub handler_block_count: u32,
    pub frame_count: u32,
    pub bb3b10_block_count: u32,

    // Hook status (DLL writes, UI reads)
    pub cave2_hooked: u32,
    pub cave1c_hooked: u32,
    pub cave1d_hooked: u32,
    pub cave5_hooked: u32,
    pub replay_capture_hooked: u32,

    // Runtime pointers (DLL internal, exposed for diagnostics)
    pub replay_ptr: u32,
    pub player_ptr: u32,

    /// Variable speed playback (1.0 = normal). UI writes, cave5 reads.
    pub playback_speed: f32,

    /// In-process restart state machine: 0 = idle, 1 = F5 held, 2 = released (done).
    pub restart_state: u32,

    // Telemetry (DLL writes, UI reads)
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    /// Ticks emitted by cave5 (the simulation rate, not the render rate).
    pub tick_count: u32,

    // Segment fields (DLL writes, UI reads)
    pub segment_start_frame: u32,
    pub segment_count: u32,
    pub segment_boundaries: [TasSegmentBoundary; TAS_MAX_SEGMENTS],

    /// 3x3 row-major rotation matrix from player+0x104..+0x124.
    pub rotation_matrix: [f32; 9],

    pub input_log: [u8; TAS_MAX_TICKS],
    pub rec_coords: [[f32; 3]; TAS_MAX_TICKS],
    pub play_coords: [[f32; 3]; TAS_MAX_TICKS],

    // Log ring buffer (DLL writes, UI reads)
    pub log_write_seq: u32,
    pub log_ring: [TasLogEntry; TAS_LOG_RING_SIZE],

    /// CONT resume speed (UI writes, DLL reads). Applied to `playback_speed`
    /// atomically at the splice. 0.0 = unset.
    pub cont_resume_speed: f32,

    /// 0 = main menu, 1 = in-game. Written each frame by cave2 from
    /// `Supreme.exe + 0x8895C`; stays 1 through a return to the menu, which
    /// only the cycle heartbeat notices.
    pub game_in_game: u32,

    /// Current track: `0..9 = area*3 + difficulty` (area 0=Forest, 1=Alpine,
    /// 2=Village, 3=Practice; diff 0=Easy, 1=Medium, 2=Hard). `u32::MAX` =
    /// unknown / menu. Trustworthy only while `level_scan_epoch == level_epoch`;
    /// read it through [`resolved_level_id`].
    pub level_id: u32,

    /// On-screen player race time in centiseconds, read by the DLL from the HUD
    /// text line. `u32::MAX` = not racing / unknown. Read via [`race_pair`]
    /// when the time and its gate stamp must come from one coherent publish.
    pub race_time_cs: u32,
    /// The 16-bit game clock value captured at the gate cross (= clock −
    /// race_time); constant during a run. `u32::MAX` = unknown.
    pub race_start_ts: u32,

    /// Test hook: when nonzero the DLL injects this exact value as the BB3B10
    /// event Time.hi (arg4) and suppresses calibration. steer-impact sets a
    /// deliberately wrong value to prove injected steering is then discarded.
    pub test_arg4_override: u32,
    /// DLL-written at each injection batch: where the injected event's Time
    /// stamp came from (`ARG4_SOURCE_*`). steer-impact asserts
    /// `ARG4_SOURCE_TIME_CURRENT`.
    pub arg4_source: u32,

    /// UI-written: 1 while a Continue cycle is in flight (from before the F5
    /// restart until the bucket aligns). The DLL blocks the real key handler
    /// whenever this is set; the level-scan worker retires a flag left set for
    /// more than 5 s of frozen cycle. Cleared by the UI the instant the bucket
    /// aligns. ESC stays exempt.
    pub cont_suppress_input: u32,

    /// Bumped by the DLL on every level-path change (a level loaded or
    /// unloaded).
    pub level_epoch: u32,
    /// The `level_epoch` the scan worker had observed when it last published a
    /// concrete `level_id`. `level_id` is trustworthy iff the two are equal.
    pub level_scan_epoch: u32,

    /// The engine's own path for the current level's resources
    /// (`Data/Levels/<Area>/<Category>/<Difficulty>/...`). Reliable for the
    /// area; the difficulty comes from the game-setup object.
    pub level_path: [u8; TAS_LEVEL_PATH_MAX],
    /// Bumped AFTER `level_path` is written.
    pub level_path_gen: u32,
    /// Seqlock over the level-context group (`level_epoch`, `level_scan_epoch`,
    /// `level_id`, `level_path`, `level_path_gen`). ODD = write in progress;
    /// the C++ side bumps it with `InterlockedIncrement`. Read the group
    /// through [`level_context`]; `with_seqlock` documents the memory model.
    pub level_ctx_seq: AtomicU32,

    /// Replay position at which the DLL drops `playback_speed` to
    /// `speed_after_handoff` on that exact tick and clears the catch-up clock
    /// backlog. 0 = no handoff. This is CONT's splice mechanism generalised so
    /// a judged PLAY can replay the countdown fast and hand back to 1x exactly
    /// where the run becomes worth watching.
    pub speed_handoff_pos: u32,
    /// Speed to assert at the handoff (0 = leave the speed alone).
    pub speed_after_handoff: f32,
    /// Bumped by cave2 every time it PROCESSES an arm that starts a replay
    /// (ARM_PLAY / ARM_CONTINUE), refusals included. The judge uses it to tell
    /// this attempt's mode/position from the previous replay's: mode is
    /// transient and position holds the previous replay's final value until
    /// the arm resets it.
    pub arm_generation: u32,
    /// `tick_count` when the in-process F5 restart completed (restart_state -> 2).
    pub restart_done_tick: u32,
    /// `tick_count` at the first captured frame whose position differs from the
    /// session's frame 0 (the countdown gate).
    pub gate_tick: u32,
    /// The REC/PLAY index at that same moment: `first_moving` as stamped by the
    /// DLL.
    pub gate_index: u32,
    /// `restart_done_tick` as it stood when THIS attempt's arm was consumed,
    /// published before `arm_generation` so the pair always comes from one
    /// attempt.
    pub arm_restart_tick: u32,
    /// The RECORDING's first-moving index. Non-zero turns on gate-relative
    /// input alignment for PLAY (`recorded_index = playback_index - gate_index +
    /// gate_align_rec`), which makes the gate index irrelevant instead of
    /// predicted. 0 leaves playback indexed from the arm.
    pub gate_align_rec: u32,
    /// 1 while every coordinate capture in this session has succeeded. A
    /// failed capture still advances the index, leaving a stale coordinate
    /// inside the prefix that a first-moving scan could read as movement.
    pub capture_ok: u32,
    /// `tick_count` at which the most recent arm was consumed.
    pub arm_consumed_tick: u32,
    /// Aligned-CONT splice interlock. The controller writes 1 when the
    /// gate-relative watcher has validated the prefix; until then cave5 parks
    /// playback at the aligned splice and cave2 refuses to splice. Cleared by
    /// ARM_CONTINUE and ClearGateAlign in the DLL. Unaligned CONT ignores it.
    pub cont_splice_approved: u32,

    /// Raw x87 control word sampled ON THE GAME THREAD every cycle: 0x007F =
    /// 24-bit precision (DirectX 6/7), 0x027F = 53-bit (OpenGL / Software2).
    /// The physics differ between the two, so recordings carry it.
    pub fpu_control_word: u32,
    /// Loaded renderer plugin, see `TAS_RENDERER_*`.
    pub renderer_id: u32,
    /// The human rider's character, see `TAS_CHARACTER_*` (0 until resolved).
    pub rider_character: u32,
    /// Stance the game builds the rider with: 0 = regular, 1 = goofy;
    /// `u32::MAX` = unknown.
    pub rider_stance: u32,
    /// Seqlock over the (rider_character, rider_stance) pair; read through
    /// [`rider_pair`].
    pub rider_seq: AtomicU32,
    /// Seqlock over the (race_time_cs, race_start_ts) pair; read through
    /// [`race_pair`].
    pub race_seq: AtomicU32,

    /// The current menu screen's on-screen title ("Main Menu", "Select
    /// Character", ...); all-zero while a level is running. Read via
    /// [`menu_screen`].
    pub menu_screen: [u8; TAS_MENU_SCREEN_MAX],

    /// Which menu item is focused, as a stable per-item id within the page
    /// (`u32::MAX` = no menu / unreadable). Creation order, not display order.
    pub menu_selector: u32,

    /// Seqlock over `menu_doc`; read the document through [`menu_doc`].
    pub menu_seq: AtomicU32,
    /// The MENU DOCUMENT: the current page's items with their visible labels
    /// and stable ids as compact JSON:
    /// `{"screen":"ID_ARCADE_MENU","sel":0,"items":[{"label":"Time Attack",
    /// "id":"ID_ARCADE_TIME_ATTACK_SEQUENCE","en":true,"vis":true},..]}`.
    /// Empty while a level runs.
    pub menu_doc: [u8; TAS_MENU_DOC_MAX],

    /// The menu COMMAND channel (agent -> DLL). Submit through
    /// [`menu_command_submit`]; the DLL executes it on the menu thread and
    /// acks the sequence in `menu_cmd_ack` after writing `menu_cmd_result`.
    /// Poll with [`menu_command_result`].
    pub menu_cmd_seq: AtomicU32,
    pub menu_cmd_kind: u32,
    pub menu_cmd_target: [u8; TAS_MENU_CMD_TARGET_MAX],
    /// The page id the command was read from; refused as STALE_PAGE if the
    /// menu moved on. Empty = unchecked.
    pub menu_cmd_screen: [u8; TAS_MENU_SCREEN_MAX],
    pub menu_cmd_ack: AtomicU32,
    pub menu_cmd_result: u32,
}

/// How many times to retry a torn seqlock read before giving up.
///
/// A writer's critical section is at most a few hundred bytes of stores, and
/// it only opens when the published value actually CHANGES — the DLL skips
/// publications that would write identical values. So the window is both short
/// and rare, and a reader that loses 64 races in a row is not racing: the
/// producer is wedged or dead. Giving up returns UNKNOWN, which every caller
/// treats as "match nothing", so exhausting the bound fails closed rather than
/// spinning a UI frame forever.
const SEQLOCK_RETRIES: usize = 64;

/// One clean read of a seqlocked group (`level_ctx_seq`, `rider_seq`,
/// `race_seq`, `menu_seq`). This is the one description of the memory model
/// behind every seqlock in this protocol; the field docs and the concurrency
/// test point back here.
///
/// `read` runs on a possibly-torn group — that is expected; the value is only
/// handed back if the sequence was EVEN before it and UNCHANGED after, which is
/// exactly the window in which no write was in flight. So `read` must not act on
/// what it sees, only collect it.
///
/// `None` means no clean window was obtained: the writer is wedged mid-update
/// (sequence stuck odd — e.g. the game crashed between the two increments) or
/// the retry bound ran out. Both are "unknown", never "assume the last value".
///
/// WHAT THIS IS AND IS NOT. The sequence is a real atomic and the ordering
/// around it is real. The PAYLOAD is not: it is read with `read_volatile` from
/// memory another PROCESS writes, which Rust's memory model does not describe at
/// all — formally a data race, and no amount of `Ordering` fixes that, because
/// the writer is outside the model. What makes it work is the target, not the
/// abstract machine: on x86 aligned byte and word accesses are indivisible,
/// loads are not reordered with loads, the mapping is cache-coherent, and the
/// DLL's `InterlockedIncrement` is a full barrier. So this is a seqlock ON X86
/// over a cross-process mapping. Do not read it as a portable one.
///
/// Freshness is also NOT provided. A clean read is coherent at the instant it
/// was taken and can be stale by the time the caller acts on it — which is why
/// the UI re-reads immediately before writing to the live buffer rather than
/// trusting its frame-start snapshot (see `stop_active_session_for_load`).
pub(crate) fn with_seqlock<T>(seq: &AtomicU32, read: impl Fn() -> T) -> Option<T> {
    for _ in 0..SEQLOCK_RETRIES {
        let s1 = seq.load(Ordering::Acquire);
        if s1 & 1 != 0 {
            std::hint::spin_loop();
            continue; // writer mid-update
        }
        let value = read();
        std::sync::atomic::fence(Ordering::Acquire);
        if seq.load(Ordering::Relaxed) == s1 {
            return Some(value);
        }
        std::hint::spin_loop(); // torn: the group changed under us
    }
    None
}

/// The live rider as ONE coherent `(character, stance)` pair (v43). Two plain
/// field reads could pair a new character with the previous stance, and a
/// recording armed in that window would carry the mixed identity for good.
/// A read that never settles reports unknown, never a guess.
pub fn rider_pair(state: &TasSharedState) -> (u32, u32) {
    with_seqlock(&state.rider_seq, || unsafe {
        (
            std::ptr::read_volatile(&state.rider_character),
            std::ptr::read_volatile(&state.rider_stance),
        )
    })
    .unwrap_or((TAS_CHARACTER_UNKNOWN, u32::MAX))
}

/// The race timer as ONE coherent `(race_time_cs, race_start_ts)` pair (v43);
/// `u32::MAX` in either half = not published / unreadable.
pub fn race_pair(state: &TasSharedState) -> (u32, u32) {
    with_seqlock(&state.race_seq, || unsafe {
        (
            std::ptr::read_volatile(&state.race_time_cs),
            std::ptr::read_volatile(&state.race_start_ts),
        )
    })
    .unwrap_or((u32::MAX, u32::MAX))
}

/// Read the group's identity half. Caller must be inside a `with_seqlock`
/// window over `level_ctx_seq`.
///
/// `None` = the context changed and the scan has not re-identified the track,
/// so `level_id` still physically holds the PREVIOUS one and must not be used.
/// Checking `level_id != 0xFFFFFFFF` is NOT equivalent: the scan publishes
/// unknown for transient reasons unrelated to a level change, and a stale id is
/// a perfectly concrete number.
fn read_identity(state: &TasSharedState) -> Option<u32> {
    // SAFETY: plain u32s in a shared mapping written by the DLL's threads.
    // Volatile so the compiler cannot cache them across the sequence loads.
    unsafe {
        let epoch = std::ptr::read_volatile(&state.level_epoch);
        let scan_epoch = std::ptr::read_volatile(&state.level_scan_epoch);
        let id = std::ptr::read_volatile(&state.level_id);
        (epoch == scan_epoch).then_some(id)
    }
}

/// A coherent read of the current track: `Some(level_id)` only when that id was
/// identified in the context we are still in, `None` while unresolved.
///
/// Reading `level_scan_epoch == level_epoch` and THEN reading `level_id`
/// separately is not sound — the level can be swapped between the two, giving
/// "resolved" plus the previous track's id. Both now happen inside one seqlock
/// window, so either the whole group is from a single publication or the read is
/// rejected.
pub fn resolved_level_id(state: &TasSharedState) -> Option<u32> {
    with_seqlock(&state.level_ctx_seq, || read_identity(state)).flatten()
}

/// Like [`resolved_level_id`], but also returns the `level_epoch` the id was
/// resolved IN, read in the SAME seqlock window.
///
/// Callers that stamp their own state with "we were resolved at epoch E" must
/// use this rather than reading `level_epoch` in a separate access: across a
/// context switch the separate read can pair the OLD track's id with the NEW
/// epoch, and the stamp then keeps asserting the old track through the very
/// switch it exists to detect.
pub fn resolved_level_id_with_epoch(state: &TasSharedState) -> Option<(u32, u32)> {
    with_seqlock(&state.level_ctx_seq, || {
        let id = read_identity(state)?;
        // SAFETY: shared mapping written by the DLL; inside the seqlock window.
        let epoch = unsafe { std::ptr::read_volatile(&state.level_epoch) };
        Some((id, epoch))
    })
    .flatten()
}

/// A coherent snapshot of the level context: `(level_id, level_path)` together.
///
/// The path is the harder half: 128 bytes that another process can be halfway
/// through rewriting, so no amount of care about the counters alone makes it
/// safe. Same window as [`resolved_level_id`], plus the bytes.
///
/// Returns `None` while unresolved OR while the writer is mid-update; a caller
/// that cannot get a clean read must treat the level as unknown, never guess.
pub fn level_context(state: &TasSharedState) -> Option<(u32, String)> {
    // Collect into a plain buffer inside the window; decode (which allocates)
    // outside it, so a retry never pays for a String it is about to discard.
    let snapshot = with_seqlock(&state.level_ctx_seq, || {
        let id = read_identity(state)?;
        let mut path = [0u8; TAS_LEVEL_PATH_MAX];
        for (i, b) in path.iter_mut().enumerate() {
            // SAFETY: as read_identity — shared mapping, written by the DLL.
            *b = unsafe { std::ptr::read_volatile(&state.level_path[i]) };
        }
        Some((id, path))
    })
    .flatten();

    let (id, path) = snapshot?;
    let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
    Some((id, String::from_utf8_lossy(&path[..end]).into_owned()))
}

/// Whether the current track is known. Prefer [`resolved_level_id`] when you
/// also need the id — this cannot express "resolved, and it is X" atomically.
pub fn level_is_resolved(state: &TasSharedState) -> bool {
    resolved_level_id(state).is_some()
}

/// `arg4_source` value meaning the injected event was stamped with the game's
/// own `Kernel::Time::Current()` (the proper, focus-independent path).
pub const ARG4_SOURCE_TIME_CURRENT: u32 = 1;

impl TasSharedState {
    pub fn mode_enum(&self) -> TasMode {
        match self.mode {
            1 => TasMode::Rec,
            2 => TasMode::Play,
            _ => TasMode::Off,
        }
    }

    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            1 => "REC",
            2 => "PLAY",
            _ => "OFF",
        }
    }

    /// Read log entries newer than `after_seq`. Returns (entries, new_cursor).
    /// The cursor should start at 0 and be updated with each call.
    ///
    /// Cursor contract: the reservation counter (`log_write_seq`) advances
    /// BEFORE a writer copies its text and publishes the slot, so a slot in
    /// range may be reserved-but-unpublished. The cursor never advances past
    /// such a slot — the read stops and retries from there next poll —
    /// otherwise a poll landing in the reserve/publish window would skip the
    /// entry forever. Slots the ring has already reused (overwritten past the
    /// retained window, or torn mid-copy) are skipped explicitly. A writer
    /// that dies mid-write stalls the cursor until the cursor resets (the UI
    /// resets it on reconnect), which is preferred to silent loss.
    pub fn read_log_entries(&self, after_seq: u32) -> (Vec<(u32, TasLogSeverity, String)>, u32) {
        let write_seq = self.log_write_seq;
        if write_seq == 0 || after_seq >= write_seq {
            return (Vec::new(), after_seq);
        }

        // Only look at the last TAS_LOG_RING_SIZE entries
        let start = write_seq.saturating_sub(TAS_LOG_RING_SIZE as u32);
        let effective_start = start.max(after_seq);

        let mut entries = Vec::new();
        let mut cursor = effective_start;
        for seq in effective_start..write_seq {
            let idx = (seq % TAS_LOG_RING_SIZE as u32) as usize;
            let entry = &self.log_ring[idx];
            // Sequence in entry is seq+1 (0 means unused)
            let published = entry.sequence;
            if published != seq + 1 {
                if published > seq + 1 {
                    // Reused past the retained window (or torn mid-copy below):
                    // seq is unrecoverable — skip it, keep going.
                    cursor = seq + 1;
                    continue;
                }
                // Reserved but not yet published: stop here and retry from
                // this seq next poll.
                break;
            }
            let severity = entry.severity_enum();
            let text = entry.text_str().to_string();
            if entry.sequence != seq + 1 {
                // Reused between the check and the copy: discard the torn
                // text; the replacement is visited at its own seq.
                cursor = seq + 1;
                continue;
            }
            entries.push((seq + 1, severity, text));
            cursor = seq + 1;
        }
        (entries, cursor)
    }
}

/// Heap-allocate a zeroed TasSharedState (avoids stack overflow for ~1.6MB struct).
/// Intended for tests across all crates in the workspace.
pub fn zeroed_boxed() -> Box<TasSharedState> {
    unsafe { Box::<TasSharedState>::new_zeroed().assume_init() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    /// The C++ side (shared_state.hpp) pins the same size and offsets with
    /// `static_assert`. Both processes map the same bytes, so a field that
    /// moves on one side only is read as garbage by the other; the offsets
    /// catch a reorder that leaves the total size unchanged.
    #[test]
    fn layout_pinned_to_shared_state_hpp() {
        use std::mem::offset_of;
        assert_eq!(mem::size_of::<TasSharedState>(), 1_651_504);
        let pins = [
            ("input_log", offset_of!(TasSharedState, input_log), 416),
            ("rec_coords", offset_of!(TasSharedState, rec_coords), 65_952),
            (
                "play_coords",
                offset_of!(TasSharedState, play_coords),
                852_384,
            ),
            (
                "log_write_seq",
                offset_of!(TasSharedState, log_write_seq),
                1_638_816,
            ),
            (
                "cont_resume_speed",
                offset_of!(TasSharedState, cont_resume_speed),
                1_647_012,
            ),
            (
                "level_ctx_seq",
                offset_of!(TasSharedState, level_ctx_seq),
                1_647_184,
            ),
            (
                "fpu_control_word",
                offset_of!(TasSharedState, fpu_control_word),
                1_647_232,
            ),
            ("menu_doc", offset_of!(TasSharedState, menu_doc), 1_647_296),
            (
                "menu_cmd_result",
                offset_of!(TasSharedState, menu_cmd_result),
                1_651_500,
            ),
        ];
        for (name, actual, expected) in pins {
            assert_eq!(actual, expected, "offset of {name}");
        }
    }

    #[test]
    fn size_of_tas_log_entry() {
        // 4 (sequence) + 4 (severity) + 120 (text) = 128
        assert_eq!(mem::size_of::<TasLogEntry>(), 128);
    }

    #[test]
    fn size_of_tas_segment_boundary() {
        assert_eq!(mem::size_of::<TasSegmentBoundary>(), 8);
    }

    /// The command words are the wire protocol cave2 switches on, and they
    /// are not contiguous.
    #[test]
    fn tas_command_round_trip() {
        let variants: &[(TasCommand, u32)] = &[
            (TasCommand::Idle, 0),
            (TasCommand::ArmRec, 1),
            (TasCommand::ArmPlay, 2),
            (TasCommand::Stop, 3),
            (TasCommand::ArmContinue, 4),
            (TasCommand::Restart, 5),
            (TasCommand::StopForRestart, 9),
        ];
        for &(cmd, val) in variants {
            assert_eq!(cmd as u32, val, "{:?} should be {}", cmd, val);
        }
    }

    /// An unknown mode word decodes to OFF rather than a guess.
    #[test]
    fn mode_decoding_defaults_unknown_to_off() {
        let mut state = zeroed_boxed();
        for (word, mode, name) in [
            (0, TasMode::Off, "OFF"),
            (1, TasMode::Rec, "REC"),
            (2, TasMode::Play, "PLAY"),
        ] {
            state.mode = word;
            assert_eq!(state.mode_enum(), mode);
            assert_eq!(state.mode_str(), name);
        }
        for bogus in [3, 42, u32::MAX] {
            state.mode = bogus;
            assert_eq!(
                state.mode_enum(),
                TasMode::Off,
                "mode {} should map to Off",
                bogus
            );
            assert_eq!(state.mode_str(), "OFF");
        }
    }

    #[test]
    fn log_entry_severity_unknown_defaults_to_info() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        entry.severity = 99;
        assert_eq!(entry.severity_enum(), TasLogSeverity::Info);
    }

    #[test]
    fn log_entry_text_str_normal() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        let msg = b"hello world";
        entry.text[..msg.len()].copy_from_slice(msg);
        assert_eq!(entry.text_str(), "hello world");
    }

    #[test]
    fn log_entry_text_str_full_buffer() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        entry.text.fill(b'A');
        assert_eq!(entry.text_str().len(), TAS_LOG_ENTRY_SIZE);
    }

    #[test]
    fn log_entry_text_str_empty() {
        let entry: TasLogEntry = unsafe { mem::zeroed() };
        assert_eq!(entry.text_str(), "");
    }

    // ========== read_log_entries: cursor semantics ==========

    fn write_log_entry(state: &mut TasSharedState, seq: u32, severity: u32, text: &str) {
        let idx = (seq % TAS_LOG_RING_SIZE as u32) as usize;
        state.log_ring[idx].sequence = seq + 1; // entry stores seq+1
        state.log_ring[idx].severity = severity;
        let bytes = text.as_bytes();
        let len = bytes.len().min(TAS_LOG_ENTRY_SIZE);
        state.log_ring[idx].text[..len].copy_from_slice(&bytes[..len]);
        if len < TAS_LOG_ENTRY_SIZE {
            state.log_ring[idx].text[len] = 0;
        }
    }

    #[test]
    fn read_log_entries_empty() {
        let state = zeroed_boxed();
        let (entries, cursor) = state.read_log_entries(0);
        assert!(entries.is_empty());
        assert_eq!(cursor, 0);
    }

    #[test]
    fn read_log_entries_single() {
        let mut state = zeroed_boxed();
        write_log_entry(&mut state, 0, 1, "first");
        state.log_write_seq = 1;

        let (entries, cursor) = state.read_log_entries(0);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, 1); // sequence stored is seq+1
        assert_eq!(entries[0].2, "first");
        assert_eq!(cursor, 1);
    }

    #[test]
    fn read_log_entries_cursor_skips_already_seen() {
        let mut state = zeroed_boxed();
        for i in 0..5u32 {
            write_log_entry(&mut state, i, 0, &format!("msg{}", i));
        }
        state.log_write_seq = 5;

        // Read from cursor=3 should only get entries 3,4
        let (entries, cursor) = state.read_log_entries(3);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].2, "msg3");
        assert_eq!(entries[1].2, "msg4");
        assert_eq!(cursor, 5);
    }

    #[test]
    fn read_log_entries_cursor_at_write_seq_returns_empty() {
        let mut state = zeroed_boxed();
        write_log_entry(&mut state, 0, 0, "msg0");
        state.log_write_seq = 1;

        let (entries, cursor) = state.read_log_entries(1);
        assert!(entries.is_empty());
        assert_eq!(cursor, 1);
    }

    #[test]
    fn read_log_entries_wraparound() {
        let mut state = zeroed_boxed();
        // Write more than ring size entries — only last 64 should be visible
        let total = TAS_LOG_RING_SIZE as u32 + 10;
        for i in 0..total {
            write_log_entry(&mut state, i, 0, &format!("w{}", i));
        }
        state.log_write_seq = total;

        // Reading from 0 should only return last 64 (ring clamps)
        let (entries, cursor) = state.read_log_entries(0);
        assert_eq!(entries.len(), TAS_LOG_RING_SIZE);
        assert_eq!(entries[0].2, "w10"); // first visible after wraparound
        assert_eq!(cursor, total);
    }

    #[test]
    fn read_log_entries_sequence_gap_skips_stale() {
        let mut state = zeroed_boxed();
        // Write entry at seq=0 but set log_write_seq=2 (gap at seq=1)
        write_log_entry(&mut state, 0, 1, "zero");
        // Don't write seq=1 — its slot has sequence=0 (stale)
        state.log_write_seq = 2;

        let (entries, cursor) = state.read_log_entries(0);
        // seq=0 matches (entry.sequence == 0+1 == 1), seq=1 doesn't (slot has sequence=0, expects 2)
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].2, "zero");
        assert_eq!(
            cursor, 1,
            "cursor stops at the unpublished slot, not past it"
        );
    }

    #[test]
    fn read_log_entries_waits_for_reserved_slot_then_reads_it() {
        let mut state = zeroed_boxed();
        // Reserve slot 0 without publishing: the counter advanced, the
        // entry's sequence did not (writer between increment and copy).
        state.log_write_seq = 1;
        let (entries, cursor) = state.read_log_entries(0);
        assert!(entries.is_empty());
        assert_eq!(
            cursor, 0,
            "cursor must not advance past an unpublished slot"
        );
        // Publish, then read again with the returned cursor: the entry
        // appears exactly once.
        write_log_entry(&mut state, 0, 1, "hello");
        let (entries, cursor) = state.read_log_entries(cursor);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].2, "hello");
        assert_eq!(cursor, 1);
        // And it is not repeated.
        let (entries, cursor) = state.read_log_entries(cursor);
        assert!(entries.is_empty());
        assert_eq!(cursor, 1);
    }
}

#[cfg(test)]
mod level_epoch_tests {
    use super::*;

    /// The distinction the epoch exists to make, which `level_id` alone cannot:
    /// an F5 restart keeps the SAME level context, a track switch does not.
    #[test]
    fn resolved_only_when_the_scan_ran_in_the_current_context() {
        let mut s = zeroed_boxed();

        // Fresh init: nothing identified, but nothing swapped either.
        s.level_epoch = 0;
        s.level_scan_epoch = 0;
        assert!(level_is_resolved(&s));

        // Scan identifies FE in this context.
        s.level_id = 0;
        assert!(level_is_resolved(&s));

        // F5 restart: the root SURVIVES, so no epoch bump — the level must stay
        // trusted. This is the case that made player_ptr the wrong signal.
        assert!(level_is_resolved(&s), "an F5 restart must not unresolve");

        // Track switch: the root is reallocated, DLL bumps the epoch. The scan
        // has not caught up, so level_id still says FE — and MUST NOT be trusted.
        s.level_epoch = 1;
        assert!(
            !level_is_resolved(&s),
            "after a context swap the stale level_id must not be trusted, even \
             though it still holds a concrete (previous) track id"
        );

        // Scan identifies the new track in the new context.
        s.level_id = 1; // FM
        s.level_scan_epoch = 1;
        assert!(level_is_resolved(&s));
        assert_eq!(crate::level::code_from_id(s.level_id), Some("FM"));
    }

    /// The snapshot read must hand back the id and its validity TOGETHER.
    /// Callers that ask "resolved?" and then separately read `level_id` can be
    /// handed "yes" plus the previous track's id if a swap lands between the
    /// two reads — that is the hole this API exists to close.
    #[test]
    fn resolved_read_returns_id_and_validity_together() {
        let mut s = zeroed_boxed();
        s.level_epoch = 7;
        s.level_scan_epoch = 7;
        s.level_id = 2; // FH, identified in this context
        assert_eq!(resolved_level_id(&s), Some(2));

        // Swap: the scan has not caught up, so level_id STILL reads FH. The
        // snapshot must refuse it rather than report a confident wrong track.
        s.level_epoch = 8;
        assert_eq!(
            resolved_level_id(&s),
            None,
            "a stale but concrete level_id must not be handed out as resolved"
        );
        assert_eq!(s.level_id, 2, "the stale id is still physically present");
    }

    /// A transient scan miss inside one context must not look like a swap.
    #[test]
    fn unknown_level_id_alone_is_not_a_context_change() {
        let mut s = zeroed_boxed();
        s.level_epoch = 3;
        s.level_scan_epoch = 3;
        s.level_id = u32::MAX; // scan found nothing this cycle

        // Resolved-but-unknown is a real state (the menu). It is NOT "the level
        // changed" — that is exactly the conflation the old code made.
        assert!(level_is_resolved(&s));
        assert_eq!(crate::level::code_from_id(s.level_id), None);
    }
}

#[cfg(test)]
mod level_seqlock_tests {
    use super::*;

    /// A reader must never observe a HALF-WRITTEN context — and the id it gets
    /// must belong to the path it gets.
    ///
    /// The path is a 128-byte array, so another process can be midway through
    /// rewriting it while the counters still read old; and `level_id` lives
    /// somewhere else entirely, so the two can disagree even when neither is
    /// itself torn. A writer thread publishes BOTH, byte-by-byte, with the
    /// sequence held odd, and the reader asserts the pair it gets is always one
    /// consistent publication.
    ///
    /// The payload is written with `write_volatile` and read with
    /// `read_volatile` from two threads, which is exactly the production
    /// reader's shape (see `with_seqlock` for why that is a seqlock on x86, not
    /// a portable one). This test models that, it does not launder it. Only
    /// `level_ctx_seq` is a real atomic, because that is the one location the
    /// protocol's correctness depends on.
    ///
    /// No `&mut TasSharedState` is ever created while the reader holds `&` —
    /// the writer goes through raw pointers via `addr_of_mut!`.
    #[test]
    fn seqlock_reader_never_sees_a_spliced_context() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        // Two publications with paths of the SAME length differing in every
        // byte, each paired with its own id, so a splice is detectable rather
        // than accidentally valid — and so an id/path mismatch is too.
        const PATH_A: &str = "data/levels/Forest/Tracks/Easy/aaaaa";
        const PATH_B: &str = "data/levels/Alpine/Tracks/Hard/bbbbb";
        const ID_A: u32 = 0; // Forest Easy
        const ID_B: u32 = 5; // Alpine Hard

        let mut boxed = zeroed_boxed();
        boxed.level_epoch = 1;
        boxed.level_scan_epoch = 1;
        let ptr = Box::into_raw(boxed) as usize;
        let stop = Arc::new(AtomicBool::new(false));

        let w_stop = stop.clone();
        let writer = std::thread::spawn(move || {
            let s = ptr as *mut TasSharedState;
            let mut which = false;
            while !w_stop.load(Ordering::Relaxed) {
                let (src, id) = if which {
                    (PATH_A.as_bytes(), ID_A)
                } else {
                    (PATH_B.as_bytes(), ID_B)
                };
                which = !which;
                unsafe {
                    // Mirror the DLL exactly: InterlockedIncrement to odd,
                    // mutate, InterlockedIncrement to even. AcqRel is what the
                    // Interlocked intrinsic gives, so the stand-in writer is not
                    // weaker than the real one.
                    (*s).level_ctx_seq.fetch_add(1, Ordering::AcqRel); // -> odd
                    std::ptr::write_volatile(std::ptr::addr_of_mut!((*s).level_id), id);
                    for (i, &c) in src.iter().enumerate() {
                        std::ptr::write_volatile(std::ptr::addr_of_mut!((*s).level_path[i]), c);
                        if i % 8 == 0 {
                            std::hint::spin_loop(); // widen the tear window
                        }
                    }
                    std::ptr::write_volatile(std::ptr::addr_of_mut!((*s).level_path[src.len()]), 0);
                    (*s).level_ctx_seq.fetch_add(1, Ordering::AcqRel); // -> even
                }
                // Leave a stable window. The real writer publishes on level
                // changes only; a back-to-back loop would hold the sequence odd
                // almost always and starve the reader, which tests nothing.
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        });

        let s = unsafe { &*(ptr as *const TasSharedState) };
        let mut clean = 0usize;
        let mut saw_a = false;
        let mut saw_b = false;
        for _ in 0..20_000 {
            if let Some((id, path)) = level_context(s) {
                if path.is_empty() {
                    continue; // pre-first-publication zeros
                }
                match (path.as_str(), id) {
                    (PATH_A, ID_A) => saw_a = true,
                    (PATH_B, ID_B) => saw_b = true,
                    other => panic!(
                        "incoherent context: {:?} — expected exactly one publication, \
                         i.e. ({:?}, {}) or ({:?}, {})",
                        other, PATH_A, ID_A, PATH_B, ID_B
                    ),
                }
                clean += 1;
            }
        }

        stop.store(true, Ordering::Relaxed);
        writer.join().unwrap();
        unsafe { drop(Box::from_raw(ptr as *mut TasSharedState)) };

        // Reads must succeed, not merely never splice — a reader that always
        // returned None would pass every assertion above. And BOTH publications
        // must have been seen, which is what proves the reader was actually
        // running concurrently with the writer rather than sampling one quiet
        // value 20,000 times.
        assert!(
            clean > 0,
            "no clean read ever completed — reader is starving"
        );
        assert!(
            saw_a && saw_b,
            "only ever saw one publication (A={} B={}) — the reader never \
             overlapped the writer, so nothing was actually tested",
            saw_a,
            saw_b
        );
    }

    /// An odd sequence means a write is in flight; the reader must refuse.
    #[test]
    fn odd_sequence_is_refused() {
        let mut s = zeroed_boxed();
        s.level_epoch = 4;
        s.level_scan_epoch = 4;
        s.level_ctx_seq.store(3, Ordering::Relaxed); // odd => mid-write
        assert_eq!(level_context(&s), None, "must not read while seq is odd");

        s.level_ctx_seq.store(4, Ordering::Relaxed); // even => stable
        assert!(level_context(&s).is_some());
    }

    /// Unresolved must stay unresolved even when the sequence is clean.
    #[test]
    fn clean_sequence_does_not_imply_resolved() {
        let mut s = zeroed_boxed();
        s.level_ctx_seq.store(8, Ordering::Relaxed);
        s.level_epoch = 5;
        s.level_scan_epoch = 4; // scan has not caught up
        s.level_id = 2;
        assert_eq!(level_context(&s), None);
    }
}
