#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../replay_capture_policy.hpp"
#include "../replay_identity.hpp"
#include "../fpu_safe_hook.hpp"

// Replay recorder capture hook at SG+0x9E8F0 (sub esp, 0x80; 6 bytes), the
// recorder's per-frame "push 112-byte frame". ECX = the recorder; the cycle
// cave derives the player from [recorder+0x84]. Only the human's recorder is
// followed (replay_capture_policy.hpp).

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

static ReplayCaptureState g_capture;

// Runs on every recorder push (every rider, every tick).
static void ReplayCaptureCb(SafetyHookContext& ctx) {
    auto* s = g_replayState;
    auto* addr = g_replayAddr;
    if (!s || !addr) return;

    auto newPtr = (uint32_t)ctx.ecx;

    static uint32_t s_logged = 0;
    ReplayIdentityEnv env{};
    env.player_vtable = addr->player_vtable;
    env.ghost_vtable = addr->ghost_vtable;
    if (newPtr != 0 && newPtr == g_capture.cached) {
        // Recheck every push: after F5 the address can be reused by a ghost.
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
        if (adopted) s->replay_ptr = newPtr;
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
}

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

    replayCaptureHook = CreateMidHook<ReplayCaptureCb>(addr.replay_capture_site);

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
