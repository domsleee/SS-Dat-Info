#pragma once
#include "stdafx.h"
#include "mapping_owner.hpp"

// Shared memory between TAS_Helper.dll and the UI; see TAS/DESIGN.md "Shared memory".
// Mirrored field for field by tas_shared/src/state.rs; bump TAS_SHARED_VERSION on any change.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";

constexpr uint32_t TAS_SHARED_VERSION = 53;
constexpr uint32_t TAS_MENU_DOC_MAX = 4096;  // menu document buffer (JSON, NUL-terminated)
constexpr uint32_t TAS_MENU_CMD_TARGET_MAX = 64;  // menu command target (id or label, NUL-terminated)
// menu_cmd_kind
constexpr uint32_t TAS_MENU_CMD_NONE = 0;
constexpr uint32_t TAS_MENU_CMD_ACTIVATE = 1;   // focus the target, then Enter on it
constexpr uint32_t TAS_MENU_CMD_FOCUS = 2;      // move the cursor only
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
constexpr uint32_t TAS_MENU_RESULT_NOT_FOCUSABLE = 6;   // focus did not land on the target
constexpr uint32_t TAS_MENU_RESULT_STALE_PAGE = 7;      // the page changed since the agent read it
constexpr uint32_t TAS_MENU_RESULT_EXPIRED = 8;         // no menu ran for 3 s
constexpr uint32_t TAS_LEVEL_PATH_MAX = 128;
constexpr uint32_t TAS_MENU_SCREEN_MAX = 32;   // menu-screen title buffer
constexpr uint32_t TAS_MAX_TICKS = 65536;
constexpr uint32_t TAS_MAX_SEGMENTS = 32;
constexpr uint32_t TAS_LOG_RING_SIZE = 64;
constexpr uint32_t TAS_LOG_ENTRY_SIZE = 120;   // text bytes per entry, incl NUL

// Commands (UI -> DLL)
enum TasCommand : uint32_t {
    CMD_IDLE         = 0,
    CMD_ARM_REC      = 1,
    CMD_ARM_PLAY     = 2,
    CMD_STOP         = 3,
    CMD_ARM_CONTINUE = 4,  // PLAY 0..continue_from_frame, then auto-switch to REC
    CMD_RESTART      = 5,  // in-process F5 restart
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

// Selectable riders (shared-state rider_character). Physics differ per rider.
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

enum TasInputBit : uint8_t {
    INPUT_LEFT  = 0x01,
    INPUT_RIGHT = 0x02,
    INPUT_UP    = 0x04,
    INPUT_DOWN  = 0x08,
    INPUT_JUMP  = 0x10,
    INPUT_SHIFT = 0x20,
};

// Where the injected BB3B10 Time stamp came from (TasSharedState::arg4_source)
enum TasArg4Source : uint32_t {
    ARG4_SOURCE_NONE         = 0,  // no injection yet
    ARG4_SOURCE_TIME_CURRENT = 1,  // Kernel::Time::Current(), the normal path
    ARG4_SOURCE_CALIBRATED   = 2,  // fallback: Time.hi observed from real keypresses
    ARG4_SOURCE_OVERRIDE     = 3,  // test_arg4_override
};

enum TasLogSeverity : uint32_t {
    LOG_DEBUG = 0,
    LOG_INFO  = 1,
    LOG_WARN  = 2,
    LOG_ERROR = 3,
};

struct TasSegmentBoundary {
    uint32_t frame;              // frame where this segment starts
};

// 128 bytes.
struct TasLogEntry {
    uint32_t  sequence;                      // monotonic, 0 = unused
    uint32_t  severity;                      // TasLogSeverity
    char      text[TAS_LOG_ENTRY_SIZE];
};

// Naturally aligned, no packing; must match the Rust repr(C) layout.
struct TasSharedState {
    uint32_t version;               // TAS_SHARED_VERSION

    // -- Command region (UI writes, DLL reads) --
    volatile uint32_t command;      // TasCommand
    uint32_t continue_from_frame;   // CMD_ARM_CONTINUE: splice point (PLAY 0..N, then REC)

    // -- Status region (DLL writes, UI reads) --
    volatile uint32_t mode;         // TasMode
    uint32_t recorded_count;        // ticks recorded
    uint32_t playback_pos;          // current playback tick
    float    player_x, player_y, player_z;

    // -- Diagnostics --
    uint32_t bb3b10_call_count;     // BB3B10 observer calls
    uint32_t handler_block_count;   // key-handler cave blocks during PLAY
    uint32_t frame_count;           // frames seen by the cycle cave
    uint32_t bb3b10_block_count;    // observer cave blocks during REC

    // -- Hook status: 1 = installed --
    uint32_t cycle_cave_hooked;
    uint32_t key_handler_cave_hooked;
    uint32_t observer_cave_hooked;
    uint32_t tick_cave_hooked;
    uint32_t replay_capture_hooked;

    // -- Runtime pointers (diagnostics) --
    uint32_t replay_ptr;
    uint32_t player_ptr;            // [replayObj+0x84]

    float    playback_speed;            // 1.0 = normal; UI writes, the tick cave reads

    // In-process restart: 0=idle, 1=F5 held, 2=done (the game rebuilt the level)
    volatile uint32_t restart_state;

    // -- Telemetry --
    float    velocity_x, velocity_y, velocity_z;            // pos - prev_pos, per frame
    uint32_t tick_count;                                    // ticks emitted by the tick cave

    // -- Segments --
    uint32_t segment_start_frame;
    uint32_t segment_count;
    TasSegmentBoundary segment_boundaries[TAS_MAX_SEGMENTS];

    uint8_t  input_log[TAS_MAX_TICKS];

    float    rec_coords[TAS_MAX_TICKS][3];   // X,Y,Z per tick (REC)
    float    play_coords[TAS_MAX_TICKS][3];  // X,Y,Z per tick (PLAY)

    // -- Log ring (DLL writes, UI reads) --
    volatile uint32_t log_write_seq;              // next sequence number
    TasLogEntry       log_ring[TAS_LOG_RING_SIZE];

    // CONT: copied into playback_speed at the splice so the resumed recording
    // does not run at the catch-up rate. 0 = unset. UI writes.
    float    cont_resume_speed;

    // 1 from a race's launch until it is left (menu, track switch), including
    // while paused. 0 at the menus.
    uint32_t game_in_game;

    // Current track: area*3 + difficulty (area 0=Forest,1=Alpine,2=Village,3=Practice;
    // diff 0=Easy,1=Medium,2=Hard). 0xFFFFFFFF = unknown / menu. Valid only
    // while level_scan_epoch == level_epoch. Written by level_context.hpp.
    uint32_t level_id;

    // Race timer, read from the HUD time line via SR_UIT Append_Text.
    // race_time_cs:  player race time in centiseconds; 0xFFFFFFFF = not racing.
    // race_start_ts: 16-bit game clock at the gate cross (clock - race_time).
    uint32_t race_time_cs;
    uint32_t race_start_ts;

    // Test hook. 0 = inject the live Kernel::Time::Current() stamp as BB3B10 arg4.
    // Nonzero = inject this value as Time.hi (lo=0) with calibration off; used
    // by the steer-impact test.
    uint32_t test_arg4_override;

    // TasArg4Source of the last injection batch (DLL writes).
    uint32_t arg4_source;

    // 1 while a restart-then-replay cycle is in flight (UI and DLL write). The
    // key-handler and observer caves block the real key handler while it is
    // set, ESC excepted. The message-pump hook clears it after >5 s of frozen
    // cycle; leaving the race clears it too.
    uint32_t cont_suppress_input;

    // level_epoch: bumped on every race launch and exit.
    // level_scan_epoch: the epoch level_id belongs to (historical name).
    // level_id is valid iff the two are equal.
    uint32_t level_epoch;
    uint32_t level_scan_epoch;

    // Engine resource path for the level (Data/Levels/<Area>/<Category>/<Difficulty>/...).
    // Reliable for the area only; some tracks share a difficulty asset.
    // level_path_gen is bumped after the string is written.
    char     level_path[TAS_LEVEL_PATH_MAX];
    uint32_t level_path_gen;
    // Seqlock over level_epoch, level_scan_epoch, level_id, level_path and
    // level_path_gen (odd = write in progress). level_context.hpp is the only writer.
    volatile uint32_t level_ctx_seq;

    // -- Gate-aligned PLAY / CONT (DESIGN.md "Replaying from the gate") --
    // Bumped by the cycle cave as the last store of every ARM_REC/ARM_PLAY/
    // ARM_CONTINUE, refusals included.
    volatile uint32_t arm_generation;
    // DLL writes: index of the first captured frame whose position differs
    // from frame 0. 0 until it fires; the arm resets it.
    volatile uint32_t gate_index;
    // UI writes before the arm: the recording's first-moving index. Nonzero
    // enables recorded_index = playback_index - gate_index + gate_align_rec.
    volatile uint32_t gate_align_rec;
    // 1 while every coordinate capture this session has succeeded.
    volatile uint32_t capture_ok;
    // CONT splice interlock: the controller writes 1 once its watcher has
    // validated the prefix. Until then playback parks at the splice. Cleared by
    // ARM_CONTINUE and ClearGateAlign; ignored when gate_align_rec == 0.
    volatile uint32_t cont_splice_approved;

    // fpu_control_word: raw x87 CW sampled every Supreme::Cycle (0x007F = 24-bit
    // under DirectX 6/7, 0x027F = 53-bit under OpenGL/Software2). renderer_id: TasRendererId.
    volatile uint32_t fpu_control_word;
    volatile uint32_t renderer_id;

    // Written on each race's first tick (rider_identity.hpp).
    // rider_character: TasCharacterId (0 = not resolved yet).
    // rider_stance: 0 = regular, 1 = goofy, 0xFFFFFFFF = unknown.
    volatile uint32_t rider_character;
    volatile uint32_t rider_stance;

    // Seqlocks (odd = mid-update): rider_seq covers the rider pair,
    // race_seq covers race_time_cs/race_start_ts.
    volatile uint32_t rider_seq;
    volatile uint32_t race_seq;

    // Menu screen title ("Main Menu", ...); empty while a level runs.
    // Written with menu_doc under menu_seq (caves/menu_cave.hpp).
    char menu_screen[TAS_MENU_SCREEN_MAX];

    // Menu document: the current page's items as compact JSON, e.g.
    //   {"screen":"ID_ARCADE_MENU","sel":0,"items":[
    //     {"label":"Time Attack","id":"ID_ARCADE_TIME_ATTACK_SEQUENCE","en":true,"vis":true}, ...]}
    // Empty while a level runs. menu_seq odd = mid-write.
    volatile uint32_t menu_seq;
    char menu_doc[TAS_MENU_DOC_MAX];

    // Menu command channel (agent -> DLL): the agent writes kind + target, then
    // bumps menu_cmd_seq. The DLL runs it on the menu thread, writes
    // menu_cmd_result, then sets menu_cmd_ack = menu_cmd_seq.
    volatile uint32_t menu_cmd_seq;
    volatile uint32_t menu_cmd_kind;      // TAS_MENU_CMD_*
    char menu_cmd_target[TAS_MENU_CMD_TARGET_MAX];
    char menu_cmd_screen[TAS_MENU_SCREEN_MAX];  // STALE_PAGE if the page changed; empty = no check
    volatile uint32_t menu_cmd_ack;
    volatile uint32_t menu_cmd_result;    // TAS_MENU_RESULT_*
};

// Rust pins the same size and group offsets; a size pin alone misses swapped fields.
#define TAS_PIN_OFFSET(field, expected) \
    static_assert(offsetof(TasSharedState, field) == (expected), \
                  "TasSharedState." #field " moved: bump TAS_SHARED_VERSION and update tas_shared/src/state.rs")
static_assert(sizeof(TasSharedState) == 1651300,
              "TasSharedState layout changed: bump TAS_SHARED_VERSION and update "
              "the Rust size pin in tas_shared/src/state.rs");
TAS_PIN_OFFSET(input_log, 240);
TAS_PIN_OFFSET(rec_coords, 65776);
TAS_PIN_OFFSET(play_coords, 852208);
TAS_PIN_OFFSET(log_write_seq, 1638640);
TAS_PIN_OFFSET(cont_resume_speed, 1646836);
TAS_PIN_OFFSET(level_ctx_seq, 1647008);
TAS_PIN_OFFSET(fpu_control_word, 1647032);
TAS_PIN_OFFSET(menu_doc, 1647092);
TAS_PIN_OFFSET(menu_cmd_result, 1651296);
#undef TAS_PIN_OFFSET

// Safe from hook callbacks (no I/O, heap, formatting or float ops) and from any thread.
inline void LogRing(TasSharedState* s, TasLogSeverity severity, const char* text) {
    if (!s) return;
    uint32_t seq = (uint32_t)InterlockedIncrement((volatile LONG*)&s->log_write_seq) - 1u;
    uint32_t idx = seq % TAS_LOG_RING_SIZE;
    TasLogEntry* entry = &s->log_ring[idx];
    entry->severity = severity;
    size_t i = 0;
    while (i < TAS_LOG_ENTRY_SIZE - 1 && text[i] != '\0') {
        entry->text[i] = text[i];
        i++;
    }
    entry->text[i] = '\0';
    entry->sequence = seq + 1;  // written last so readers see a complete entry; 0 = unused
}

// DLL side: creates the mapping.
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

        memset(state, 0, sizeof(TasSharedState));
        state->version = TAS_SHARED_VERSION;
        state->playback_speed = 1.0f;
        state->level_id = 0xFFFFFFFFu;        // equal epochs + unknown id = "at the menu"
        state->race_time_cs = 0xFFFFFFFFu;
        state->race_start_ts = 0xFFFFFFFFu;
        state->rider_stance = 0xFFFFFFFFu;
        state->arg4_source = ARG4_SOURCE_NONE;
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
