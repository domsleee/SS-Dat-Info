#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../replay_capture_policy.hpp"
#include "../replay_identity.hpp"
#include "../external/safetyhook.hpp"

// Replay object capture hook at SG+0x9E8F0.
// Original instruction: sub esp, 00000080 (6 bytes).
// At this site, ECX holds the replay recorder object (the function is the
// recorder's per-frame "push 112-byte frame"). We capture it so Cave 2 can
// derive the player pointer from [recorder+0x84] every cycle.
//
// Only the HUMAN's recorder is followed (see replay_identity.hpp /
// replay_capture_policy.hpp): the one whose owner is a plain `Player` that still links
// back to it. Ghost / AI recorders that reach this site around a restart are
// ignored, and a re-created human recorder is adopted immediately, even
// mid-run.

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
static char* ReplayPut(char* p, const char* s) {
    while (*s) *p++ = *s++;
    return p;
}

// The recorder the DLL follows. File scope so a reinstall starts clean
// instead of trusting a pointer from a previous process life.
static ReplayCaptureState g_capture;

bool InstallReplayCapture(GameAddresses& addr, TasSharedState* state) {
    if (!addr.replay_capture_site) {
        Log("Replay capture: hook site not resolved");
        return false;
    }
    if (!addr.player_vtable) {
        Log("Replay capture: Player vtable not resolved - cannot identify the human rider");
        return false;
    }

    g_replayState = state;
    g_replayAddr = &addr;
    g_capture = ReplayCaptureState{};
    state->replay_ptr = 0;
    state->player_ptr = 0;
    Log(std::format("Replay capture: hooking at {:p} (SG+0x9E8F0), human = Player vtable {:#010x}",
                    (void*)addr.replay_capture_site, addr.player_vtable));

    replayCaptureHook = safetyhook::create_mid(addr.replay_capture_site, [](SafetyHookContext& ctx) {
        uint64_t t0 = __rdtsc();
        auto* s = g_replayState;
        auto* addr = g_replayAddr;
        if (!s || !addr) return;

        // ECX holds the recorder object at this hook site.
        auto newPtr = (uint32_t)ctx.ecx;

        static uint32_t s_logged = 0;
        ReplayIdentityEnv env{};
        env.player_vtable = addr->player_vtable;
        env.ghost_vtable = addr->ghost_vtable;
        if (newPtr != 0 && newPtr == g_capture.cached) {
            // Same address as the recorder we follow: prove it is STILL the
            // human's on every push (three guarded reads). An F5 can free it
            // and the allocator can hand the address to a ghost.
            const ReplayOwnerKind kind = ClassifyRecorderOwner(newPtr, env, ReplaySafeReadU32, nullptr);
            if (ReplayCaptureRevalidate(kind == OWNER_HUMAN, g_capture)) {
                s->replay_ptr = 0;
                s->player_ptr = 0;
                if (++s_logged <= 60) {
                    char msg[96];
                    char* p = ReplayPut(msg, "replay-capture ecx=");
                    ReplayHexU32(p, newPtr);
                    p = ReplayPut(p + 8, " now ");
                    p = ReplayPut(p, ReplayOwnerKindName(kind));
                    p = ReplayPut(p, " - DROPPED (address reused)");
                    *p = 0;
                    LogRing(s, LOG_DEBUG, msg);
                }
            }
        } else if (newPtr != g_capture.cached) {
            ReplayIdentityTrace trace{};
            const ReplayOwnerKind kind = ClassifyRecorderOwner(newPtr, env, ReplaySafeReadU32, &trace);
            const bool human = kind == OWNER_HUMAN;
            const uint32_t rejectedBefore = g_capture.rejected;
            const bool adopted = ReplayCaptureAdopt(s->mode == MODE_OFF, newPtr, human, g_capture);
            if (adopted) {
                s->replay_ptr = newPtr;
            }
            // Ring-log adoptions and rejections (rate-limited) with the owner
            // and its class: this is how the ghost/AI behaviour around
            // restarts was established, keep it visible.
            if ((adopted || g_capture.rejected != rejectedBefore) && ++s_logged <= 60) {
                char msg[128];
                char* p = ReplayPut(msg, "replay-capture ecx=");
                ReplayHexU32(p, newPtr);
                p = ReplayPut(p + 8, " owner=");
                ReplayHexU32(p, trace.owner);
                p = ReplayPut(p + 8, " vt=");
                ReplayHexU32(p, trace.owner_vtable);
                p = ReplayPut(p + 8, " ");
                p = ReplayPut(p, ReplayOwnerKindName(kind));
                p = ReplayPut(p, adopted
                    ? ((s->mode == MODE_OFF) ? " adopted (idle)" : " adopted (MID-RUN re-creation)")
                    : " ignored");
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
