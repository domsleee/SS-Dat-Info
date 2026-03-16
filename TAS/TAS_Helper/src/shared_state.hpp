#pragma once
#include "stdafx.h"

// Shared memory layout between TAS_Helper.dll and the egui UI process.
// Lock-free single-writer: UI writes command region, DLL writes status region.
// Both use atomic uint32_t for command/mode fields.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";
constexpr uint32_t TAS_SHARED_VERSION = 1;
constexpr uint32_t TAS_MAX_TICKS = 65536;

// Commands (UI -> DLL)
enum TasCommand : uint32_t {
    CMD_IDLE     = 0,
    CMD_ARM_REC  = 1,
    CMD_ARM_PLAY = 2,
    CMD_STOP     = 3,
};

// Modes (DLL -> UI)
enum TasMode : uint32_t {
    MODE_OFF  = 0,
    MODE_REC  = 1,
    MODE_PLAY = 2,
};

// All fields are naturally aligned (uint32_t/float = 4 bytes).
// No packing needed; must match Rust repr(C) layout.
struct TasSharedState {
    // -- Version (read-only after init) --
    uint32_t version;               // TAS_SHARED_VERSION

    // -- Command region (UI writes, DLL reads) --
    volatile uint32_t command;      // TasCommand
    uint32_t inject_mode;           // 6 = proven mode
    uint32_t force_fixed_tick;      // 0=off, 2=deterministic
    uint32_t force_direct;          // 2=BB3B10 direct calls
    uint32_t self_capture;          // 0=false
    uint32_t use_rec_msg_args;      // 1=use recorded wrapper args
    uint32_t input_source;          // 0=DI buffer sample

    // -- Status region (DLL writes, UI reads) --
    volatile uint32_t mode;         // TasMode
    uint32_t recorded_count;        // ticks recorded
    uint32_t playback_pos;          // current playback tick
    uint32_t prev_mask;             // last input bitmask
    uint32_t cave2_injecting;       // playback injection active
    float    player_x, player_y, player_z;
    float    max_drift_x, max_drift_z;

    // -- Diagnostics --
    uint32_t bb3b10_call_count;
    uint32_t handler_block_count;
    uint32_t frame_count;
    uint32_t event_count;

    // -- Hook status (DLL writes, UI reads) --
    uint32_t cave2_hooked;          // 1 if Supreme::Cycle hook installed
    uint32_t cave1c_hooked;         // 1 if handler gate hooks installed
    uint32_t cave1d_hooked;         // 1 if BB3B10 observer hook installed
    uint32_t cave5_hooked;          // 1 if fixed tick hook installed

    // Padding to align input_log
    uint32_t _reserved[8];

    // -- Input log (DLL reads/writes) --
    uint8_t  input_log[TAS_MAX_TICKS];

    // -- Coordinate logs --
    float    rec_coords[TAS_MAX_TICKS][3];   // X,Y,Z per tick (REC)
    float    play_coords[TAS_MAX_TICKS][3];  // X,Y,Z per tick (PLAY)
};

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
        state->force_fixed_tick = 2;    // deterministic
        state->force_direct = 2;        // BB3B10 direct calls
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
