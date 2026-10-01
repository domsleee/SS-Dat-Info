//! The wire layout pinned field by field: each entry fixes a field's type
//! and offset at compile time, and every field must be listed. The test
//! builds the manifest from these pins and compares it with
//! `shared_layout.txt`, which TAS_Helper's test_shared_layout.cpp checks the
//! C++ side against. On any change: bump `TAS_SHARED_VERSION` and update
//! both sides' pins and the manifest.

use std::sync::atomic::AtomicU32;

use crate::state::*;

macro_rules! pin_layout {
    ($($s:ident ($size:literal, $align:literal) { $($f:ident: $t:ty = $off:literal,)* })*) => {
        $(
            const _: () = assert!(
                std::mem::size_of::<$s>() == $size && std::mem::align_of::<$s>() == $align,
                concat!(stringify!($s), " size changed: bump TAS_SHARED_VERSION, update the pins")
            );
            // No `..`: a field missing from the list does not compile.
            const _: fn(&$s) = |s| {
                let $s { $($f: _,)* } = s;
            };
            $(
                const _: () = assert!(
                    std::mem::offset_of!($s, $f) == $off,
                    concat!(stringify!($s), ".", stringify!($f), " moved: bump TAS_SHARED_VERSION, update the pins")
                );
                // The field's type and array shape.
                const _: fn(&$s) -> &$t = |s| &s.$f;
            )*
        )*

        #[cfg(test)]
        fn manifest_layout() -> Vec<String> {
            use std::mem::{align_of, offset_of, size_of};
            let mut lines = Vec::new();
            $(lines.push(format!("struct {} {} {}", stringify!($s), size_of::<$s>(), align_of::<$s>()));)*
            $($(lines.push(format!(
                "field {}.{} {} {} {} {}",
                stringify!($s),
                stringify!($f),
                <$t as tests::Wire>::name(),
                offset_of!($s, $f),
                size_of::<$t>(),
                align_of::<$t>(),
            ));)*)*
            lines
        }
    };
}

pin_layout! {
    TasSegmentBoundary (4, 4) {
        frame: u32 = 0,
    }
    TasLogEntry (128, 4) {
        sequence: u32 = 0,
        severity: u32 = 4,
        text: [u8; TAS_LOG_ENTRY_SIZE] = 8,
    }
    TasSharedState (1651444, 4) {
        version: u32 = 0,
        command: u32 = 4,
        continue_from_frame: u32 = 8,
        mode: u32 = 12,
        recorded_count: u32 = 16,
        playback_pos: u32 = 20,
        player_x: f32 = 24,
        player_y: f32 = 28,
        player_z: f32 = 32,
        bb3b10_call_count: u32 = 36,
        handler_block_count: u32 = 40,
        frame_count: u32 = 44,
        bb3b10_block_count: u32 = 48,
        cycle_cave_hooked: u32 = 52,
        key_handler_cave_hooked: u32 = 56,
        observer_cave_hooked: u32 = 60,
        tick_cave_hooked: u32 = 64,
        replay_capture_hooked: u32 = 68,
        replay_ptr: u32 = 72,
        player_ptr: u32 = 76,
        playback_speed: f32 = 80,
        restart_state: u32 = 84,
        velocity_x: f32 = 88,
        velocity_y: f32 = 92,
        velocity_z: f32 = 96,
        tick_count: u32 = 100,
        segment_start_frame: u32 = 104,
        segment_count: u32 = 108,
        segment_boundaries: [TasSegmentBoundary; TAS_MAX_SEGMENTS] = 112,
        input_log: [u8; TAS_MAX_TICKS] = 240,
        rec_coords: [[f32; 3]; TAS_MAX_TICKS] = 65776,
        play_coords: [[f32; 3]; TAS_MAX_TICKS] = 852208,
        log_write_seq: u32 = 1638640,
        log_ring: [TasLogEntry; TAS_LOG_RING_SIZE] = 1638644,
        cont_resume_speed: f32 = 1646836,
        game_in_game: u32 = 1646840,
        level_id: u32 = 1646844,
        race_time_cs: u32 = 1646848,
        race_start_ts: u32 = 1646852,
        test_arg4_override: u32 = 1646856,
        arg4_source: u32 = 1646860,
        cont_suppress_input: u32 = 1646864,
        level_epoch: u32 = 1646868,
        level_scan_epoch: u32 = 1646872,
        level_path: [u8; TAS_LEVEL_PATH_MAX] = 1646876,
        level_path_gen: u32 = 1647004,
        level_ctx_seq: AtomicU32 = 1647008,
        arm_generation: u32 = 1647012,
        gate_index: u32 = 1647016,
        gate_align_rec: u32 = 1647020,
        capture_ok: u32 = 1647024,
        cont_splice_approved: u32 = 1647028,
        fpu_control_word: u32 = 1647032,
        renderer_id: u32 = 1647036,
        rider_character: u32 = 1647040,
        rider_stance: u32 = 1647044,
        rider_seq: AtomicU32 = 1647048,
        race_seq: AtomicU32 = 1647052,
        menu_screen: [u8; TAS_MENU_SCREEN_MAX] = 1647056,
        menu_seq: AtomicU32 = 1647088,
        menu_doc: [u8; TAS_MENU_DOC_MAX] = 1647092,
        menu_cmd_seq: AtomicU32 = 1651188,
        menu_cmd_kind: u32 = 1651192,
        menu_cmd_target: [u8; TAS_MENU_CMD_TARGET_MAX] = 1651196,
        menu_cmd_screen: [u8; TAS_MENU_SCREEN_MAX] = 1651260,
        menu_cmd_ack: AtomicU32 = 1651292,
        menu_cmd_result: u32 = 1651296,
        owner_request_seq: AtomicU32 = 1651300,
        owner_request_kind: u32 = 1651304,
        owner_request_pid: u32 = 1651308,
        owner_request_created_lo: u32 = 1651312,
        owner_request_created_hi: u32 = 1651316,
        owner_ack_seq: AtomicU32 = 1651320,
        owner_result: u32 = 1651324,
        owner_pid: u32 = 1651328,
        owner_generation: u32 = 1651332,
        game_call: u32 = 1651336,
        crash_seq: AtomicU32 = 1651340,
        crash_pid: u32 = 1651344,
        crash_code: u32 = 1651348,
        crash_address: u32 = 1651352,
        crash_game_call: u32 = 1651356,
        crash_thread_id: u32 = 1651360,
        crash_module_offset: u32 = 1651364,
        crash_module: [u8; TAS_CRASH_MODULE_MAX] = 1651368,
        game_exit_clean: u32 = 1651400,
        race_clock_bits: u32 = 1651404,
        race_clock_flags: u32 = 1651408,
        race_ab_cs: u32 = 1651412,
        race_ab_bits: u32 = 1651416,
        input_model: u32 = 1651420,
        race_finish_seq: AtomicU32 = 1651424,
        race_finish_tick: u32 = 1651428,
        race_finish_mode: u32 = 1651432,
        race_finish_valid: u32 = 1651436,
        race_finish_time_bits: u32 = 1651440,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    /// Wire type names, in the manifest's spelling.
    pub(super) trait Wire {
        fn name() -> String;
    }
    macro_rules! wire {
        ($($t:ty => $name:literal),*) => {
            $(impl Wire for $t {
                fn name() -> String {
                    $name.into()
                }
            })*
        };
    }
    wire!(u8 => "u8", u32 => "u32", AtomicU32 => "u32", f32 => "f32",
          TasSegmentBoundary => "TasSegmentBoundary", TasLogEntry => "TasLogEntry");
    impl<T: Wire, const N: usize> Wire for [T; N] {
        fn name() -> String {
            format!("[{};{}]", T::name(), N)
        }
    }

    /// The values both sides define, under their C++ names.
    fn manifest_consts() -> Vec<String> {
        macro_rules! consts {
            ($($name:ident $(= $value:expr)?),* $(,)?) => {
                vec![$(format!("const {} {}", stringify!($name), consts!(@v $name $($value)?))),*]
            };
            (@v $name:ident) => { $name as u64 };
            (@v $name:ident $value:expr) => { $value as u64 };
        }
        use input_bits::*;
        let mut lines = vec![format!(
            "const TAS_SHARED_MEMORY_NAME {TAS_SHARED_MEMORY_NAME}"
        )];
        lines.extend(consts![
            TAS_SHARED_VERSION,
            TAS_MENU_DOC_MAX,
            TAS_MENU_CMD_TARGET_MAX,
            TAS_CRASH_MODULE_MAX,
            TAS_LEVEL_PATH_MAX,
            TAS_MENU_SCREEN_MAX,
            TAS_MAX_TICKS,
            TAS_MAX_SEGMENTS,
            TAS_LOG_RING_SIZE,
            TAS_LOG_ENTRY_SIZE,
            CMD_IDLE = TasCommand::Idle,
            CMD_ARM_REC = TasCommand::ArmRec,
            CMD_ARM_PLAY = TasCommand::ArmPlay,
            CMD_STOP = TasCommand::Stop,
            CMD_ARM_CONTINUE = TasCommand::ArmContinue,
            CMD_RESTART = TasCommand::Restart,
            CMD_STOP_FOR_RESTART = TasCommand::StopForRestart,
            CMD_TEST_FAULT = TasCommand::TestFault,
            MODE_OFF = TasMode::Off,
            MODE_REC = TasMode::Rec,
            MODE_PLAY = TasMode::Play,
            INPUT_LEFT = LEFT,
            INPUT_RIGHT = RIGHT,
            INPUT_UP = UP,
            INPUT_DOWN = DOWN,
            INPUT_JUMP = JUMP,
            INPUT_SHIFT = SHIFT,
            TAS_INPUT_MODEL_INJECTED,
            TAS_INPUT_MODEL_HELD,
            TAS_RACE_CLOCK_STARTED,
            TAS_RACE_CLOCK_FINISHED,
            ARG4_SOURCE_TIME_CURRENT,
            LOG_DEBUG = TasLogSeverity::Debug,
            LOG_INFO = TasLogSeverity::Info,
            LOG_WARN = TasLogSeverity::Warn,
            LOG_ERROR = TasLogSeverity::Error,
            TAS_MENU_CMD_ACTIVATE,
            TAS_MENU_CMD_FOCUS,
            TAS_MENU_CMD_UP,
            TAS_MENU_CMD_DOWN,
            TAS_MENU_CMD_LEFT,
            TAS_MENU_CMD_RIGHT,
            TAS_MENU_CMD_TRIGGER,
            TAS_MENU_RESULT_OK,
            TAS_MENU_RESULT_NO_MENU,
            TAS_MENU_RESULT_NOT_FOUND,
            TAS_MENU_RESULT_DISABLED,
            TAS_MENU_RESULT_BAD_KIND,
            TAS_MENU_RESULT_FAULT,
            TAS_MENU_RESULT_NOT_FOCUSABLE,
            TAS_MENU_RESULT_STALE_PAGE,
            TAS_MENU_RESULT_EXPIRED,
            TAS_OWNER_ACQUIRE,
            TAS_OWNER_RELEASE,
            TAS_OWNER_RESULT_OWNED,
            TAS_OWNER_RESULT_RELEASED,
            TAS_OWNER_RESULT_BUSY,
            TAS_OWNER_RESULT_NO_PROCESS,
            TAS_OWNER_RESULT_WRONG_PROCESS,
            TAS_OWNER_RESULT_NOT_OWNER,
            TAS_OWNER_RESULT_BAD_KIND,
            TAS_GAME_CALL_NONE,
            TAS_GAME_CALL_MENU_TRIGGER,
            TAS_GAME_CALL_MENU_MOVE,
            TAS_GAME_CALL_MENU_FOCUS,
            TAS_GAME_CALL_MENU_ACTIVE,
            TAS_GAME_CALL_TIME_CURRENT,
            TAS_GAME_CALL_TEST_FAULT,
            TAS_GAME_CALL_OBSERVER_FLUSH,
            TAS_RENDERER_UNKNOWN,
            TAS_RENDERER_DIRECTX6,
            TAS_RENDERER_DIRECTX7,
            TAS_RENDERER_OPENGL,
            TAS_RENDERER_GLIDE3X,
            TAS_RENDERER_SOFTWARE2,
            TAS_CHARACTER_UNKNOWN,
            TAS_CHARACTER_KEITH,
            TAS_CHARACTER_VINCENT,
            TAS_CHARACTER_AKIKO,
            TAS_CHARACTER_KARL,
            TAS_CHARACTER_MIKE,
            TAS_CHARACTER_ULRIKA,
            TAS_CHARACTER_OTHER,
        ]);
        lines
    }

    /// Both sides map the same bytes, so a field that moves or changes type on
    /// one side only is read as garbage by the other. The pins above fix this
    /// side; the shared manifest makes CI compare it with the C++ side.
    #[test]
    fn layout_matches_the_manifest() {
        let ours: Vec<String> = manifest_layout()
            .into_iter()
            .chain(manifest_consts())
            .collect();
        let file: Vec<&str> = include_str!("../shared_layout.txt")
            .lines()
            .map(str::trim_end)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        if let Some(i) = (0..ours.len().max(file.len()))
            .find(|&i| ours.get(i).map(String::as_str) != file.get(i).copied())
        {
            panic!(
                "shared_layout.txt disagrees with the Rust pins at entry {i}:\n  Rust: {:?}\n  file: {:?}\nRust manifest:\n{}",
                ours.get(i),
                file.get(i),
                ours.join("\n")
            );
        }
    }
}
