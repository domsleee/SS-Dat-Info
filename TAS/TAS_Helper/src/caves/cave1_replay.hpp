#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../replay_capture_policy.hpp"
#include "../external/safetyhook.hpp"

// Replay object capture hook at SG+0x9E8F0.
// Original instruction: sub esp, 00000080 (6 bytes).
// At this site, ECX holds the replay recorder object (the function is the
// recorder's per-frame "push 112-byte frame"). We capture it so Cave 2 can
// derive the player pointer from [recorder+0x84] every cycle.
//
// Only the HUMAN's recorder is followed (see replay_capture_policy.hpp): its
// owner player is the one whose controller holds the keyboard object. Ghost /
// AI recorders that reach this site around a restart are ignored, and a
// re-created human recorder is adopted immediately, even mid-run.

inline TasSharedState* g_replayState = nullptr;
inline GameAddresses* g_replayAddr = nullptr;
static SafetyHookMid replayCaptureHook{};

inline void UninstallReplayCapture() {
    replayCaptureHook = {};
    if (g_replayState) g_replayState->replay_capture_hooked = 0;
    g_replayState = nullptr;
    g_replayAddr = nullptr;
}

// Hook-safe helpers (no CRT, no C++ objects).
static uint32_t ReplaySafeReadU32(uint32_t addr) {
    if (addr < 0x10000) return 0;
    __try {
        return *(volatile uint32_t*)addr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
static void ReplayHexU32(char* dst, uint32_t v) {
    static const char H[] = "0123456789ABCDEF";
    for (int i = 7; i >= 0; i--) {
        dst[i] = H[v & 0xF];
        v >>= 4;
    }
}

// Is `recorder` owned by the keyboard-driven (human) player?
//   owner      = [recorder + 0x84]
//   controller = [owner + 0x1B8]
//   [controller + 0x590] == [root + 0x530]  (the keyboard object)
static bool ReplayRecorderIsHuman(uint32_t recorder, uint32_t* ownerOut) {
    uint32_t owner = ReplaySafeReadU32(recorder + GameAddresses::REPLAY_PLAYER_OFFSET);
    if (ownerOut) *ownerOut = owner;
    if (!owner || !g_replayAddr) return false;
    uint32_t root = ReplaySafeReadU32((uint32_t)g_replayAddr->player_base);
    uint32_t kb = ReplaySafeReadU32(root + GameAddresses::KEYBOARD_OBJ_OFFSET);
    if (!kb) return false;
    uint32_t controller = ReplaySafeReadU32(owner + GameAddresses::PLAYER_CONTROLLER_OFFSET);
    return ReplaySafeReadU32(controller + GameAddresses::CONTROLLER_KEYBOARD_OFFSET) == kb;
}

bool InstallReplayCapture(GameAddresses& addr, TasSharedState* state) {
    if (!addr.replay_capture_site) {
        Log("Replay capture: hook site not resolved");
        return false;
    }

    g_replayState = state;
    g_replayAddr = &addr;
    Log(std::format("Replay capture: hooking at {:p} (SG+0x9E8F0)", (void*)addr.replay_capture_site));

    replayCaptureHook = safetyhook::create_mid(addr.replay_capture_site, [](SafetyHookContext& ctx) {
        uint64_t t0 = __rdtsc();
        auto* s = g_replayState;
        if (!s) return;

        // ECX holds the recorder object at this hook site.
        auto newPtr = (uint32_t)ctx.ecx;

        static ReplayCaptureState s_capture;
        static uint32_t s_logged = 0;
        if (newPtr != s_capture.cached) {
            uint32_t owner = 0;
            const bool human = ReplayRecorderIsHuman(newPtr, &owner);
            const uint32_t rejectedBefore = s_capture.rejected;
            const bool adopted = ReplayCaptureAdopt(s->mode == MODE_OFF, newPtr, human, s_capture);
            if (adopted) {
                s->replay_ptr = newPtr;
            }
            // Ring-log adoptions and rejections (rate-limited) with the owner
            // player: this is how the ghost/AI behaviour around restarts was
            // established, keep it visible.
            if ((adopted || s_capture.rejected != rejectedBefore) && ++s_logged <= 40) {
                char msg[96] = "replay-capture ecx=";
                char* p = msg + 19;
                ReplayHexU32(p, newPtr);
                p += 8;
                const char* tag = " owner=";
                while (*tag) *p++ = *tag++;
                ReplayHexU32(p, owner);
                p += 8;
                const char* tail = adopted
                    ? ((s->mode == MODE_OFF) ? " adopted (idle)" : " adopted (MID-RUN re-creation)")
                    : " ignored (not the keyboard-driven player)";
                while (*tail) *p++ = *tail++;
                *p = 0;
                LogRing(s, LOG_DEBUG, msg);
            }
        }

        PerfSample(s->perf_replay_capture, __rdtsc() - t0);
    });

    if (!replayCaptureHook) {
        Log("Replay capture: SafetyHook create_mid FAILED");
        g_replayState = nullptr;
        g_replayAddr = nullptr;
        return false;
    }

    state->replay_capture_hooked = 1;
    Log("Replay capture: hook installed successfully");
    return true;
}
