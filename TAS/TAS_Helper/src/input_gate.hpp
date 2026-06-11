#pragma once
#include <cstdint>

// Pure decision logic for the keyboard-handler gate (cave1c), extracted from
// the SafetyHook detours so it can be unit-tested WITHOUT the Windows / hook
// context (see src/tests/test_input_gate.cpp). cave1c's detours build an
// InputGateInputs from the live call and act on ShouldBlockRealInput; keeping
// the policy here means the held-key-during-CONT regression has a guard.
//
// "Block" = swallow the real key event so it never reaches the game's handler
// (and thus never writes the DI buffer / fires the BB3B10 observer).

struct InputGateInputs {
    uint32_t mode;       // TasMode: 0=OFF, 1=REC, 2=PLAY
    bool cont_suppress;  // shared cont_suppress_input: a Continue is in flight
    bool injecting;      // cave2_injecting: our OWN injected call (must pass)
    bool game_paused;    // cycle heartbeat stale (>250ms): pause menu/dialog/reload
    bool is_escape;      // the event is ESC (always exempt: abort hatch + menu nav)
};

// TasMode::Off as a plain constant (avoid pulling shared_state.hpp into tests).
inline constexpr uint32_t INPUT_GATE_MODE_OFF = 0;

inline bool ShouldBlockRealInput(const InputGateInputs& in) {
    // Our own injected calls always pass (cave2 owns the DI buffer + observer).
    if (in.injecting) return false;
    // ESC always passes: it's the abort hatch and drives pause-menu nav.
    if (in.is_escape) return false;

    // CONT block — takes PRECEDENCE over the pause passthrough. While a
    // Continue is in flight, live input must be inert through the WHOLE
    // restart/replay, INCLUDING the F5 reload and post-finish/dialog state
    // that legitimately stall Supreme::Cycle (game_paused). A held/pressed key
    // reaching the spawn then corrupts the restart (observed: ctrl/Enter held
    // during a post-finish CONT → F5 spawned the boarder at the finish line,
    // drift 2315 at tick 1). Deliberately ignores game_paused — that is the
    // whole point of this branch existing separately from the mode block.
    if (in.cont_suppress) return true;

    // REC/PLAY symmetry block — pause-EXEMPT so the pause menu (and a
    // stuck-mode session) stays navigable: when the sim isn't running there's
    // no REC/PLAY timing symmetry to protect, and the menu's own navigation is
    // handler/observer-driven.
    if (in.mode != INPUT_GATE_MODE_OFF && !in.game_paused) return true;

    return false;
}
