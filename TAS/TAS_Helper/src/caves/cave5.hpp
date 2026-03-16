#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 5: Fixed tick override.
// Game computes (int)(elapsed_seconds * 100), clamped 0-20.
// This cave forces the tick count to a fixed value (typically 2) for determinism.
//
// Hook site is at the tick computation result, after __ftol.
// Pattern scan needed to find it (varies by binary).
// Phase 1: stub only. Phase 2 will implement the actual override.

static SafetyHookMid cave5Hook{};

bool InstallCave5(GameAddresses& addr, TasSharedState* state) {
    // Cave 5 needs pattern scanning - it's not at a fixed offset.
    // The tick clamp is at +25C81/+426000 area.
    // For Phase 1, we just verify we can find the pattern.

    if (!addr.sg) {
        Log("Cave 5: Supreme_Game.dll not loaded");
        return false;
    }

    // Pattern for the tick computation site (from CE analysis):
    // The game does: mov esi, eax (result of ftol) then cmp esi, 14h (clamp to 20)
    // We'll scan for the clamping sequence.
    // This is a Phase 2 task - for now just log that we'd hook here.

    Log("Cave 5: stub - tick override hook deferred to Phase 2");
    Log("  (tick clamp at SG+25C81/+426000 area, needs pattern scan)");

    // Mark as not hooked yet
    state->cave5_hooked = 0;
    return true;  // Not a failure - just deferred
}
