#pragma once
#include "stdafx.h"
#include "mapping_owner.hpp"

// Shared memory layout between TAS_Helper.dll and the egui UI process.
// Lock-free single-writer: UI writes the command region, DLL writes the status
// region. The Rust mirror is tas_shared/src/lib.rs; both pin the size and the
// group-boundary offsets below, and TAS_SHARED_VERSION is bumped on any change.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";

constexpr uint32_t TAS_SHARED_VERSION = 51;
constexpr uint32_t TAS_MENU_DOC_MAX = 4096;  // menu document buffer (JSON, NUL-terminated)
constexpr uint32_t TAS_MENU_CMD_TARGET_MAX = 64;  // menu command target (id or label, NUL-terminated)
// menu_cmd_kind
constexpr uint32_t TAS_MENU_CMD_NONE = 0;
constexpr uint32_t TAS_MENU_CMD_ACTIVATE = 1;   // focus the target, then Enter on it
constexpr uint32_t TAS_MENU_CMD_FOCUS = 2;      // just move the cursor to the target
constexpr uint32_t TAS_MENU_CMD_UP = 3;
constexpr uint32_t TAS_MENU_CMD_DOWN = 4;
constexpr uint32_t TAS_MENU_CMD_LEFT = 5;
constexpr uint32_t TAS_MENU_CMD_RIGHT = 6;
constexpr uint32_t TAS_MENU_CMD_TRIGGER = 7;    // Enter on whatever is focused
// menu_cmd_result
constexpr uint32_t TAS_MENU_RESULT_OK = 0;
constexpr uint32_t TAS_MENU_RESULT_NO_MENU = 1;
constexpr uint32_t TAS_MENU_RESULT_NOT_FOUND = 2;
constexpr uint32_t TAS_MENU_RESULT_DISABLED = 3;
constexpr uint32_t TAS_MENU_RESULT_BAD_KIND = 4;
constexpr uint32_t TAS_MENU_RESULT_FAULT = 5;
constexpr uint32_t TAS_MENU_RESULT_NOT_FOCUSABLE = 6;   // the focus did not land on the target; nothing triggered
constexpr uint32_t TAS_MENU_RESULT_STALE_PAGE = 7;      // the page moved on since the agent read it; nothing done
constexpr uint32_t TAS_MENU_RESULT_EXPIRED = 8;         // no menu was executing for 3 s; nothing done
constexpr uint32_t TAS_LEVEL_PATH_MAX = 128;
constexpr uint32_t TAS_MENU_SCREEN_MAX = 32;   // menu-screen title buffer
constexpr uint32_t TAS_MAX_TICKS = 65536;
constexpr uint32_t TAS_MAX_SEGMENTS = 32;      // Max segment boundaries
constexpr uint32_t TAS_LOG_RING_SIZE = 64;     // Number of log entries
constexpr uint32_t TAS_LOG_ENTRY_SIZE = 120;   // Max text bytes per entry (incl NUL)

// Commands (UI -> DLL)
enum TasCommand : uint32_t {
    CMD_IDLE         = 0,
    CMD_ARM_REC      = 1,
    CMD_ARM_PLAY     = 2,
    CMD_STOP         = 3,
    CMD_ARM_CONTINUE = 4,  // PLAY 0..continue_from_frame, then auto-switch to REC
    CMD_RESTART      = 5,  // In-process F5 restart (no Pico/focus needed)
    CMD_STOP_FOR_RESTART = 9, // internal stop: protect live input across restart
};

// Renderer plugin loaded by sr.dll (shared-state renderer_id).
enum TasRendererId : uint32_t {
    TAS_RENDERER_UNKNOWN   = 0,
    TAS_RENDERER_DIRECTX6  = 1,
    TAS_RENDERER_DIRECTX7  = 2,
    TAS_RENDERER_OPENGL    = 3,
    TAS_RENDERER_GLIDE3X   = 4,
    TAS_RENDERER_SOFTWARE2 = 5,
};

// Selectable riders (shared-state rider_character). The physics differ per
// character, so recordings are stamped with this.
enum TasCharacterId : uint32_t {
    TAS_CHARACTER_UNKNOWN = 0,
    TAS_CHARACTER_KEITH   = 1,
    TAS_CHARACTER_VINCENT = 2,
    TAS_CHARACTER_AKIKO   = 3,
    TAS_CHARACTER_KARL    = 4,
    TAS_CHARACTER_MIKE    = 5,
    TAS_CHARACTER_ULRIKA  = 6,
    TAS_CHARACTER_OTHER   = 7,   // a name outside this table (mod / custom rider)
};

// Modes (DLL -> UI)
enum TasMode : uint32_t {
    MODE_OFF  = 0,
    MODE_REC  = 1,
    MODE_PLAY = 2,
};

// Input mask bit definitions
enum TasInputBit : uint8_t {
    INPUT_LEFT  = 0x01,  // bit 0
    INPUT_RIGHT = 0x02,  // bit 1
    INPUT_UP    = 0x04,  // bit 2
    INPUT_DOWN  = 0x08,  // bit 3
    INPUT_JUMP  = 0x10,  // bit 4
    INPUT_SHIFT = 0x20,  // bit 5
};

// Where the injected BB3B10 Time stamp came from (TasSharedState::arg4_source)
enum TasArg4Source : uint32_t {
    ARG4_SOURCE_NONE         = 0,  // no injection yet
    ARG4_SOURCE_TIME_CURRENT = 1,  // Kernel::Time::Current() — the proper path
    ARG4_SOURCE_CALIBRATED   = 2,  // fallback: Time.hi observed from real keypresses
    ARG4_SOURCE_OVERRIDE     = 3,  // test_arg4_override forced (steer-impact test)
};

// Log severity levels
enum TasLogSeverity : uint32_t {
    LOG_DEBUG = 0,
    LOG_INFO  = 1,
    LOG_WARN  = 2,
    LOG_ERROR = 3,
};

// A segment boundary record (8 bytes)
struct TasSegmentBoundary {
    uint32_t frame;              // Frame number where this segment starts
    uint32_t input_log_offset;   // Offset into input_log for this segment
};

// A single log ring entry (128 bytes, naturally aligned)
struct TasLogEntry {
    uint32_t  sequence;                      // Monotonic counter (0 = unused)
    uint32_t  severity;                      // TasLogSeverity
    char      text[TAS_LOG_ENTRY_SIZE];      // NUL-terminated message
};

// All fields are naturally aligned 4-byte words. No packing; must match the Rust repr(C) layout.
struct TasSharedState {
    // -- Version (read-only after init) --
    uint32_t version;               // TAS_SHARED_VERSION

    // -- Command region (UI writes, DLL reads) --
    volatile uint32_t command;      // TasCommand
    uint32_t force_fixed_tick;      // 0 = natural ticks, N = force N ticks/frame (cave5)
    uint32_t continue_from_frame;   // CMD_ARM_CONTINUE: splice point (PLAY 0..N, then REC)

    // -- Status region (DLL writes, UI reads) --
    volatile uint32_t mode;         // TasMode
    uint32_t recorded_count;        // ticks recorded
    uint32_t playback_pos;          // current playback tick
    float    player_x, player_y, player_z;
    float    max_drift_x, max_drift_z;

    // -- Diagnostics --
    uint32_t bb3b10_call_count;     // BB3B10 observer calls (Cave 2 direct)
    uint32_t handler_block_count;   // Cave 1C: external handler blocks during PLAY
    uint32_t frame_count;           // Cave 2: total frames processed
    uint32_t bb3b10_block_count;    // Cave 1D: BB3B10 blocks during REC

    // -- Hook status (DLL writes, UI reads) --
    uint32_t cave2_hooked;          // 1 if Supreme::Cycle hook installed
    uint32_t cave1c_hooked;         // 1 if handler gate hooks installed
    uint32_t cave1d_hooked;         // 1 if BB3B10 observer hook installed
    uint32_t cave5_hooked;          // 1 if fixed tick hook installed
    uint32_t replay_capture_hooked; // 1 if replay object capture hook installed

    // -- Runtime pointers (DLL internal, exposed for diagnostics) --
    uint32_t replay_ptr;            // Current replay object pointer
    uint32_t player_ptr;            // Current player pointer ([replayObj+0x84])

    // Variable speed playback (1.0 = normal, 0.5 = half, 2.0 = double)
    float    playback_speed;            // UI writes, Cave 5 reads

    // In-process restart state machine: 0=idle, 1=F5 held, 2=released (done)
    volatile uint32_t restart_state;

    // -- Telemetry (DLL writes, UI reads) --
    float    velocity_x, velocity_y, velocity_z;            // Per-frame velocity (pos - prev_pos)
    uint32_t tick_count;                                    // Ticks emitted by Cave 5

    // -- Segment fields (DLL writes, UI reads) --
    uint32_t segment_start_frame;    // Frame offset where the current segment begins
    uint32_t segment_count;          // Total segments in the current run
    TasSegmentBoundary segment_boundaries[TAS_MAX_SEGMENTS]; // (frame, offset) pairs

    // -- Rotation telemetry (DLL writes, UI reads) --
    float rotation_matrix[9]; // 3x3 row-major rotation matrix from player+0x104..+0x124

    // -- Input log (DLL reads/writes) --
    uint8_t  input_log[TAS_MAX_TICKS];

    // -- Coordinate logs --
    float    rec_coords[TAS_MAX_TICKS][3];   // X,Y,Z per tick (REC)
    float    play_coords[TAS_MAX_TICKS][3];  // X,Y,Z per tick (PLAY)

    // -- Log ring buffer (DLL writes, UI reads) --
    volatile uint32_t log_write_seq;              // Next sequence number to write (monotonic)
    TasLogEntry       log_ring[TAS_LOG_RING_SIZE]; // Circular buffer of log entries

    // -- CONT resume speed (UI writes, DLL reads) --
    // Applied to playback_speed atomically at the splice so the resumed
    // recording doesn't fast-forward at the catch-up rate. 0 = unset.
    float    cont_resume_speed;

    // -- Game-state awareness (DLL writes each frame from exe+0x8895C) --
    // 0 = main menu, 1 = in-game (in a race/level). Note: stays 1 through a
    // return to the menu; the cycle heartbeat is what notices that.
    uint32_t game_in_game;

    // -- Current track (DLL level-scan worker writes; UI reads) --
    // 0..9 = area*3 + difficulty (area 0=Forest,1=Alpine,2=Village,3=Practice;
    // diff 0=Easy,1=Medium,2=Hard). 0xFFFFFFFF = unknown / menu. Trustworthy
    // only while level_scan_epoch == level_epoch (see below).
    uint32_t level_id;

    // -- Race timer (DLL reads the HUD time line via SR_UIT Append_Text) --
    // race_time_cs  = on-screen player race time in centiseconds (exact,
    //                 map-agnostic). 0xFFFFFFFF = not racing / unknown.
    // race_start_ts = 16-bit game clock value captured at the gate cross
    //                 (= clock - race_time); constant during a run.
    uint32_t race_time_cs;
    uint32_t race_start_ts;

    // -- Test hook: force the injected BB3B10 arg4 (config) --
    // 0 = normal (inject the live Kernel::Time::Current() stamp). Nonzero =
    // inject this EXACT value as Time.hi (lo=0) and suppress all calibration.
    // The steer-impact test drives a deliberately-wrong value (injection must
    // be discarded) then 0 (live stamp, boarder turns) to prove the event Time
    // is load-bearing.
    uint32_t test_arg4_override;

    // -- Injection Time-stamp source (DLL writes at each injection batch) --
    // TasArg4Source. steer-impact asserts ARG4_SOURCE_TIME_CURRENT so the
    // proper mechanism can't silently regress to the fallback.
    uint32_t arg4_source;

    // -- Live-input suppression during a CONT (UI writes, DLL reads) --
    // 1 while a Continue cycle is in flight (from before the F5 restart until
    // the bucket aligns). cave1c/cave1d block the real key handler whenever it
    // is set, covering the OFF-mode spawn countdown the mode-based block
    // misses. The level-scan worker retires a flag left set for >5 s of frozen
    // cycle (a cycle that never tore down), which would otherwise leave the
    // keyboard dead. ESC stays exempt.
    uint32_t cont_suppress_input;

    // -- Level context epoch (DLL level-scan worker writes; UI reads) --
    //   level_epoch:      bumped on every level-path change (a new level loaded
    //                     or the level unloaded).
    //   level_scan_epoch: the epoch the worker had observed when it last
    //                     published a CONCRETE level_id.
    // level_id is trustworthy iff level_scan_epoch == level_epoch; otherwise
    // the context changed and the new track is not identified yet.
    uint32_t level_epoch;
    uint32_t level_scan_epoch;

    // -- Level path (DLL-written, event-driven) --
    // The engine's own path for the current level's resources
    // (Data/Levels/<Area>/<Category>/<Difficulty>/...). Reliable for the area;
    // some tracks share a difficulty asset, so the difficulty comes from the
    // game-setup object instead. level_path_gen is bumped AFTER the string is
    // written.
    char     level_path[TAS_LEVEL_PATH_MAX];
    uint32_t level_path_gen;
    // Seqlock over the level-context group (level_epoch, level_scan_epoch,
    // level_id, level_path, level_path_gen): ODD = write in progress. The path
    // is a 128-byte array, so a reader in the other process can see it half
    // copied unless it takes the sequence before and after and accepts only an
    // unchanged EVEN value. level_scan.hpp is the only writer.
    volatile uint32_t level_ctx_seq;

    // -- Judged PLAY / CONT controls (UI writes, cave2/cave5 read) --
    // Replay position at which cave2 drops playback_speed to
    // speed_after_handoff on that exact tick and asks cave5 to clear the
    // catch-up backlog. 0 = no handoff.
    volatile uint32_t speed_handoff_pos;
    volatile float speed_after_handoff;
    // Bumped every time cave2 PROCESSES a replay-starting arm (ARM_PLAY /
    // ARM_CONTINUE), refusals included. The judge uses it to tell this
    // attempt's mode/position from the previous replay's.
    volatile uint32_t arm_generation;
    // Countdown gate. tick_count when restart_state -> 2; tick_count and
    // REC/PLAY index of the first captured frame whose position differs from
    // frame 0 (the boarder leaving the spawn); restart_done_tick latched at
    // this attempt's arm (published before arm_generation so the pair never
    // straddles two attempts).
    volatile uint32_t restart_done_tick;
    volatile uint32_t gate_tick;
    volatile uint32_t gate_index;
    volatile uint32_t arm_restart_tick;
    // The RECORDING's first-moving index; non-zero enables gate-relative input
    // alignment for PLAY (recorded_index = playback_index - gate_index +
    // gate_align_rec). 0 = index from the arm.
    volatile uint32_t gate_align_rec;
    // 1 while every coordinate capture this session has succeeded. A failed
    // capture still advances the index, leaving a stale hole in the prefix.
    volatile uint32_t capture_ok;
    // tick_count at which the most recent arm was consumed.
    volatile uint32_t arm_consumed_tick;
    // Aligned-CONT splice interlock. Written 1 by the controller when the
    // gate-relative watcher has validated the prefix. Until then cave5 parks
    // playback at the aligned splice (0 ticks/frame) and cave2 refuses to
    // splice. Cleared by ARM_CONTINUE and ClearGateAlign. Unaligned CONT
    // (gate_align_rec == 0) ignores it.
    volatile uint32_t cont_splice_approved;

    // -- Renderer / x87 precision (DLL writes) --
    // fpu_control_word: raw x87 CW sampled on the game thread every
    // Supreme::Cycle (0x007F = 24-bit under DirectX 6/7, 0x027F = 53-bit under
    // OpenGL/Software2; the physics differ). renderer_id: TasRendererId.
    volatile uint32_t fpu_control_word;
    volatile uint32_t renderer_id;

    // -- Rider identity (DLL level-scan worker writes) --
    // rider_character: TasCharacterId (0 = not resolved yet).
    // rider_stance: 0 = regular, 1 = goofy, 0xFFFFFFFF = unknown. Read from the
    // game-setup object; cannot be switched in-process.
    volatile uint32_t rider_character;
    volatile uint32_t rider_stance;

    // Seqlocks for the two published PAIRS (odd = writer mid-update):
    //   rider_seq - (rider_character, rider_stance)
    //   race_seq  - (race_time_cs, race_start_ts)
    volatile uint32_t rider_seq;
    volatile uint32_t race_seq;

    // -- Menu screen title ("Main Menu", "Select Character", ...); empty while
    // a level runs. Captured by the race timer's SR_UIT text hook. Not
    // seqlocked: a torn read is a one-frame cosmetic blip.
    char menu_screen[TAS_MENU_SCREEN_MAX];

    // The focused menu item's index within the current page (0xFFFFFFFF = no
    // menu / unreadable). See menu_state.hpp.
    volatile uint32_t menu_selector;

    // The MENU DOCUMENT: the current page's items with visible labels and
    // stable ids as compact JSON, e.g.
    //   {"screen":"ID_ARCADE_MENU","sel":0,"items":[
    //     {"label":"Time Attack","id":"ID_ARCADE_TIME_ATTACK_SEQUENCE","en":true,"vis":true}, ...]}
    // Empty while a level runs. Written under menu_seq (odd = mid-write).
    volatile uint32_t menu_seq;
    char menu_doc[TAS_MENU_DOC_MAX];

    // The MENU COMMAND channel (agent -> DLL). The agent writes kind + target
    // (+ the page it read the target from), then bumps menu_cmd_seq. The DLL
    // executes it on the menu thread through the game's own entry points,
    // writes menu_cmd_result, then sets menu_cmd_ack = menu_cmd_seq.
    volatile uint32_t menu_cmd_seq;
    volatile uint32_t menu_cmd_kind;      // TAS_MENU_CMD_*
    char menu_cmd_target[TAS_MENU_CMD_TARGET_MAX];
    char menu_cmd_screen[TAS_MENU_SCREEN_MAX];  // refused with STALE_PAGE if the page moved on; empty = no check
    volatile uint32_t menu_cmd_ack;
    volatile uint32_t menu_cmd_result;    // TAS_MENU_RESULT_*
};

// The C++ and Rust views of this struct map the same shared memory from two
// processes, so both pin the total size AND the offsets of the group
// boundaries (a size pin alone cannot catch two fields swapped). Bump
// TAS_SHARED_VERSION whenever any of these numbers changes.
#define TAS_PIN_OFFSET(field, expected) \
    static_assert(offsetof(TasSharedState, field) == (expected), \
                  "TasSharedState." #field " moved: bump TAS_SHARED_VERSION and update tas_shared/src/lib.rs")
static_assert(sizeof(TasSharedState) == 1651504,
              "TasSharedState layout changed: bump TAS_SHARED_VERSION and update "
              "the Rust size pin in tas_shared/src/lib.rs");
TAS_PIN_OFFSET(input_log, 416);
TAS_PIN_OFFSET(rec_coords, 65952);
TAS_PIN_OFFSET(play_coords, 852384);
TAS_PIN_OFFSET(log_write_seq, 1638816);
TAS_PIN_OFFSET(cont_resume_speed, 1647012);
TAS_PIN_OFFSET(level_ctx_seq, 1647184);
TAS_PIN_OFFSET(fpu_control_word, 1647232);
TAS_PIN_OFFSET(menu_doc, 1647296);
TAS_PIN_OFFSET(menu_cmd_result, 1651500);
#undef TAS_PIN_OFFSET

// Write a log entry to the ring buffer. Safe to call from hook callbacks
// (no file I/O, no heap allocation, no printf/format) and from any thread:
// the slot is claimed with an interlocked increment.
// text must be a NUL-terminated C string.
inline void LogRing(TasSharedState* s, TasLogSeverity severity, const char* text) {
    if (!s) return;
    uint32_t seq = (uint32_t)InterlockedIncrement((volatile LONG*)&s->log_write_seq) - 1u;
    uint32_t idx = seq % TAS_LOG_RING_SIZE;
    TasLogEntry* entry = &s->log_ring[idx];
    entry->severity = severity;
    // Safe string copy without x87/float ops
    size_t i = 0;
    while (i < TAS_LOG_ENTRY_SIZE - 1 && text[i] != '\0') {
        entry->text[i] = text[i];
        i++;
    }
    entry->text[i] = '\0';
    // Write sequence last (acts as release fence for reader)
    entry->sequence = seq + 1;  // +1 so 0 means "unused"
}

// Shared memory management (DLL side - creates the mapping)
class TasSharedMemory {
    MappingOwner owner;
public:
    HANDLE hMapFile = nullptr;
    TasSharedState* state = nullptr;

    bool Create() {
        if (state) return true;
        if (!owner.Acquire("Local\\SupremeTAS.Owner")) return false;
        hMapFile = CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            nullptr,
            PAGE_READWRITE,
            0,
            sizeof(TasSharedState),
            TAS_SHARED_MEMORY_NAME
        );
        if (!hMapFile) {
            owner.Release();
            return false;
        }

        state = (TasSharedState*)MapViewOfFile(
            hMapFile,
            FILE_MAP_ALL_ACCESS,
            0, 0,
            sizeof(TasSharedState)
        );
        if (!state) {
            CloseHandle(hMapFile);
            hMapFile = nullptr;
            owner.Release();
            return false;
        }

        // Zero-init, then the non-zero defaults.
        memset(state, 0, sizeof(TasSharedState));
        state->version = TAS_SHARED_VERSION;
        state->playback_speed = 1.0f;
        state->level_id = 0xFFFFFFFFu;        // unknown until the scan thread runs
        state->race_time_cs = 0xFFFFFFFFu;
        state->race_start_ts = 0xFFFFFFFFu;
        state->rider_stance = 0xFFFFFFFFu;    // unknown until the setup object is read
        state->menu_selector = 0xFFFFFFFFu;   // no menu selection until the menu publishes one
        state->arg4_source = ARG4_SOURCE_NONE;
        // level_epoch == level_scan_epoch with level_id unknown reads as
        // "trustworthy but unknown" (at the menu / not yet scanned).
        return true;
    }

    void Destroy() {
        if (state) {
            UnmapViewOfFile(state);
            state = nullptr;
        }
        if (hMapFile) {
            CloseHandle(hMapFile);
            hMapFile = nullptr;
        }
        owner.Release();
    }

    ~TasSharedMemory() { Destroy(); }
};
