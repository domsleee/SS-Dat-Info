#pragma once
#include "../shared_state.hpp"
#include "../owner_os_win32.hpp"
#include "cycle_cave.hpp"

// Controller ownership, run from the message pump (DESIGN.md "Controller
// ownership"). The live-input block belongs to the owner: it ends with the
// ownership, and when the owner exits mid-operation the TAS stops.
namespace owner {

inline OwnerTracker g_tracker{owner_os::kWin32};

static void Publish(TasSharedState* s) {
    s->owner_pid = g_tracker.pid();
    s->owner_generation = g_tracker.generation();
}

// The owner exited without releasing: whatever it was driving is abandoned.
static void AbortForGoneOwner(TasSharedState* s) {
    LogRing(s, LOG_ERROR, "The controller driving the TAS exited: stopped the TAS");
    ApplyStopTransition(s, false);
    // Its unconsumed command must not run for nobody.
    InterlockedExchange((volatile LONG*)&s->command, CMD_IDLE);
}

static void HandleRequest(TasSharedState* s) {
    const uint32_t seq = s->owner_request_seq;
    if (seq == s->owner_ack_seq) return;
    MemoryBarrier();  // fields were written before the sequence
    const uint64_t created =
        ((uint64_t)s->owner_request_created_hi << 32) | s->owner_request_created_lo;
    const uint32_t result =
        g_tracker.Handle(s->owner_request_kind, s->owner_request_pid, created);
    if (result == OWNER_RESULT_RELEASED) s->cont_suppress_input = 0;
    s->owner_result = result;
    Publish(s);
    InterlockedExchange((volatile LONG*)&s->owner_ack_seq, (LONG)seq);
}

// Game thread, every pump pass.
inline void Process(TasSharedState* s) {
    // Before the request, so a successor is not refused as BUSY by an owner
    // that already exited.
    if (g_tracker.OwnerGone()) {
        AbortForGoneOwner(s);
        Publish(s);
    }
    HandleRequest(s);
    if (s->cont_suppress_input && g_tracker.pid() == 0) {
        s->cont_suppress_input = 0;
        LogRing(s, LOG_WARN, "Cleared a live-input block that no controller owns");
    }
}

inline void Uninstall() { g_tracker.Drop(); }

}  // namespace owner
