#pragma once
#include <cstdint>

// The key-handler cave's block policy, kept free of hook code so
// src/tests/test_input_gate.cpp can test it. Blocking swallows the real key
// event before it reaches the game's handler.

struct InputGateInputs {
    uint32_t mode;       // TasMode: 0=OFF, 1=REC, 2=PLAY
    bool cont_suppress;  // cont_suppress_input: a CONT is in flight
    bool injecting;      // this thread is inside our injected call
    bool game_paused;    // cycle heartbeat older than 250 ms
    bool is_escape;
};

// Thread-scoped, so a real key event on another thread can't slip through
// during injection.
inline thread_local uint32_t g_tasInjectionDepth = 0;

inline bool IsTasInjectionThread() {
    return g_tasInjectionDepth != 0;
}

// TasMode::Off, without pulling shared_state.hpp into tests.
inline constexpr uint32_t INPUT_GATE_MODE_OFF = 0;

inline bool ShouldBlockRealInput(const InputGateInputs& in) {
    if (in.injecting) return false;
    // ESC is the abort key and drives the pause menu.
    if (in.is_escape) return false;

    // A CONT blocks even while paused: the F5 reload stalls the cycle, and a
    // key held into the restart can move the spawn.
    if (in.cont_suppress) return true;

    // REC/PLAY block, lifted while paused so menus stay usable.
    if (in.mode != INPUT_GATE_MODE_OFF && !in.game_paused) return true;

    return false;
}
