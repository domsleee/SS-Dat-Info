#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../replay_capture_policy.hpp"
#include "../replay_identity.hpp"
#include "../menu_model.hpp"
#include "../safe_read.hpp"
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

static ReplayCaptureState g_capture;

// Runs on every recorder push (every rider, every tick).
static void ReplayCaptureCb(SafetyHookContext& ctx) {
    auto* s = g_replayState;
    auto* addr = g_replayAddr;
    if (!s || !addr) return;

    auto newPtr = (uint32_t)ctx.ecx;

    static uint32_t s_logged = 0;
    char msg[TAS_LOG_ENTRY_SIZE];
    menumodel::TextWriter w{msg, sizeof msg};
    if (newPtr != 0 && newPtr == g_capture.cached) {
        // Recheck every push: after F5 the address can be reused by a ghost.
        const ReplayOwnerKind kind =
            ClassifyRecorderOwner(newPtr, addr->player_vtable, SafeRead32, nullptr);
        if (ReplayCaptureRevalidate(kind == OWNER_HUMAN, g_capture)) {
            s->replay_ptr = 0;
            s->player_ptr = 0;
            if (++s_logged <= 60) {
                w.Put("replay-capture ecx=");
                w.PutHex(newPtr);
                w.Put(" now ");
                w.Put(ReplayOwnerKindName(kind));
                w.Put(" - DROPPED (address reused)");
                LogRing(s, LOG_DEBUG, w.Finish());
            }
        }
    } else if (newPtr != g_capture.cached) {
        ReplayIdentityTrace trace{};
        const ReplayOwnerKind kind =
            ClassifyRecorderOwner(newPtr, addr->player_vtable, SafeRead32, &trace);
        const bool human = kind == OWNER_HUMAN;
        const uint32_t rejectedBefore = g_capture.rejected;
        const bool adopted = ReplayCaptureAdopt(newPtr, human, g_capture);
        if (adopted) s->replay_ptr = newPtr;
        if ((adopted || g_capture.rejected != rejectedBefore) && ++s_logged <= 60) {
            w.Put("replay-capture ecx=");
            w.PutHex(newPtr);
            w.Put(" owner=");
            w.PutHex(trace.owner);
            w.Put(" vt=");
            w.PutHex(trace.owner_vtable);
            w.Put(" ");
            w.Put(ReplayOwnerKindName(kind));
            w.Put(adopted
                ? ((s->mode == MODE_OFF) ? " adopted (idle)" : " adopted (MID-RUN re-creation)")
                : " ignored");
            LogRing(s, LOG_DEBUG, w.Finish());
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
