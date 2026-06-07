#pragma once
#include "stdafx.h"

// Shared memory layout between TAS_Helper.dll and the egui UI process.
// Lock-free single-writer: UI writes command region, DLL writes status region.
// Both use atomic uint32_t for command/mode fields.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";
constexpr uint32_t TAS_SHARED_VERSION = 7;  // Phase 7: game_in_game field (DLL must populate it)
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
    CMD_SNAPSHOT     = 6,  // PROTOTYPE: capture writable memory snapshot at this frame
    CMD_RESTORE      = 7,  // PROTOTYPE: restore the last snapshot (instant rewind)
    CMD_SNAPSHOT_AT_SPAWN = 8, // PROTOTYPE: arm a snapshot at the next PLAY frame-0 (spawn)
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

// Hook performance counters (cycles measured with __rdtsc).
struct TasHookPerfCounter {
    uint64_t calls;          // Number of callback invocations
    uint64_t cycles_total;   // Sum of elapsed cycles across all calls
    uint64_t cycles_max;     // Worst single-call elapsed cycles
};

// All fields are naturally aligned (uint32_t/float = 4 bytes).
// No packing needed; must match Rust repr(C) layout.
struct TasSharedState {
    // -- Version (read-only after init) --
    uint32_t version;               // TAS_SHARED_VERSION

    // -- Command region (UI writes, DLL reads) --
    volatile uint32_t command;      // TasCommand
    uint32_t inject_mode;           // 6 = proven mode
    uint32_t force_fixed_tick;      // 0=natural ticks (proven zero-drift), N=force N ticks/frame
    uint32_t force_direct;          // 2=BB3B10 direct calls
    uint32_t self_capture;          // 0=false
    uint32_t use_rec_msg_args;      // 1=use recorded wrapper args
    uint32_t input_source;          // 0=DI buffer sample
    uint32_t continue_from_frame;   // CMD_ARM_CONTINUE: splice point (PLAY 0..N, then REC)

    // -- Status region (DLL writes, UI reads) --
    volatile uint32_t mode;         // TasMode
    uint32_t recorded_count;        // ticks recorded
    uint32_t playback_pos;          // current playback tick
    uint32_t prev_mask;             // last input bitmask
    uint32_t cave2_injecting;       // playback injection active (Cave 1C/1D gate flag)
    float    player_x, player_y, player_z;
    float    max_drift_x, max_drift_z;

    // -- Diagnostics --
    uint32_t bb3b10_call_count;     // BB3B10 observer calls (Cave 2 direct)
    uint32_t handler_block_count;   // Cave 1C: external handler blocks during PLAY
    uint32_t frame_count;           // Cave 2: total frames processed
    uint32_t event_count;           // General event counter
    uint32_t bb3b10_block_count;    // Cave 1D: BB3B10 blocks during REC mode 6

    // -- Hook performance counters (DLL writes, UI/tests read) --
    TasHookPerfCounter perf_cave2;
    TasHookPerfCounter perf_cave5;
    TasHookPerfCounter perf_cave1c_down;
    TasHookPerfCounter perf_cave1c_up;
    TasHookPerfCounter perf_cave1d;
    TasHookPerfCounter perf_replay_capture;

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

    // In-process restart state machine (DLL internal)
    // 0=idle, 1=F5 pressed (waiting frames), 2=F5 released (done)
    volatile uint32_t restart_state;
    uint32_t restart_frames_held;       // Frames F5 has been held down

    // -- Telemetry (DLL writes, UI reads) --
    float    prev_player_x, prev_player_y, prev_player_z;  // Previous frame position
    float    velocity_x, velocity_y, velocity_z;            // Per-frame velocity (pos - prev_pos)
    float    speed;                                          // Magnitude of velocity XZ
    uint32_t tick_count;                                    // Current tick count from Cave 5

    // -- Segment fields (DLL writes, UI reads) --
    uint32_t segment_index;          // Current segment number (0-based)
    uint32_t segment_start_frame;    // Frame offset where current segment begins
    uint32_t segment_count;          // Total segments in current run
    uint32_t snapshot_size;          // Bytes in last boundary snapshot
    uint32_t snapshot_flags;         // Bitmask: bit0=player, bit1=physics, bit2=fpu, bit3=rand_seed
    uint32_t snapshot_buffer_ptr;    // External snapshot allocation pointer
    uint32_t snapshot_buffer_capacity; // Total snapshot buffer size
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

    // -- CONT splice timing (DLL writes, harness/UI reads) --
    // frame_count stamped at CONT replay start and at the PLAY->REC splice; the
    // delta = game-frames the catch-up replay took (the "resume off by a few
    // frames" measurement). Must stay last to match the Rust struct layout.
    uint32_t cont_replay_start_fc;
    uint32_t cont_splice_fc;

    // -- CONT resume speed (UI writes, DLL reads) --
    // Applied to playback_speed atomically at the splice so the resumed
    // recording doesn't fast-forward at the catch-up rate. 0 = unset.
    float    cont_resume_speed;

    // -- CONT clock-backlog reset (cave2 sets at splice, cave5 consumes) --
    uint32_t cont_reset_pending;

    // -- Game-state awareness (DLL writes each frame from exe+0x8895C) --
    // 0 = main menu, 1 = in-game (in a race/level). Lets the UI know the game
    // state and lets cave5 avoid touching ticks in the menu. Fills the tail
    // padding so sizeof is unchanged (1_647_248).
    uint32_t game_in_game;
};

// Write a log entry to the ring buffer. Safe to call from hook callbacks
// (no file I/O, no heap allocation, no printf/format).
// text must be a NUL-terminated C string.
inline void LogRing(TasSharedState* s, TasLogSeverity severity, const char* text) {
    if (!s) return;
    uint32_t seq = s->log_write_seq++;
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

inline void PerfSample(TasHookPerfCounter& c, uint64_t elapsedCycles) {
    c.calls++;
    c.cycles_total += elapsedCycles;
    if (elapsedCycles > c.cycles_max) {
        c.cycles_max = elapsedCycles;
    }
}

inline void ResetHookPerfCounters(TasSharedState* s) {
    if (!s) return;
    memset(&s->perf_cave2, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave5, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1c_down, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1c_up, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1d, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_replay_capture, 0, sizeof(TasHookPerfCounter));
}

// Shared memory management (DLL side - creates the mapping)
class TasSharedMemory {
public:
    HANDLE hMapFile = nullptr;
    TasSharedState* state = nullptr;

    bool Create() {
        hMapFile = CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            nullptr,
            PAGE_READWRITE,
            0,
            sizeof(TasSharedState),
            TAS_SHARED_MEMORY_NAME
        );
        if (!hMapFile) return false;

        state = (TasSharedState*)MapViewOfFile(
            hMapFile,
            FILE_MAP_ALL_ACCESS,
            0, 0,
            sizeof(TasSharedState)
        );
        if (!state) {
            CloseHandle(hMapFile);
            hMapFile = nullptr;
            return false;
        }

        // Zero-init and set version
        memset(state, 0, sizeof(TasSharedState));
        state->version = TAS_SHARED_VERSION;
        state->inject_mode = 6;        // proven default
        state->force_fixed_tick = 0;    // natural ticks (proven zero-drift config)
        state->force_direct = 2;        // BB3B10 direct calls
        state->use_rec_msg_args = 1;   // proven zero-drift config
        state->playback_speed = 1.0f;  // normal speed
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
    }

    ~TasSharedMemory() { Destroy(); }
};
