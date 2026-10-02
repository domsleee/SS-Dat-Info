#pragma once
#include <cstdint>

// The key-handler cave's block policy, kept free of hook code so
// src/tests/test_input_gate.cpp can test it. Blocking swallows the real key
// event before it reaches the game's handler.

struct InputGateInputs {
    uint32_t mode;       // TasMode: 0=OFF, 1=REC, 2=PLAY
    bool cont_suppress;  // cont_suppress_input: a CONT is in flight
    bool injecting;      // this thread is inside our injected call
    bool game_paused;    // IsMenuPause: a menu runs over a stalled race
    bool is_escape;
    bool is_tas_key;     // one of the six keys a take records
    bool is_release;     // a key-up
};

// Thread-scoped, so a real key event on another thread can't slip through
// during injection.
inline thread_local uint32_t g_tasInjectionDepth = 0;

inline bool IsTasInjectionThread() {
    return g_tasInjectionDepth != 0;
}

// The PLAY block lifts only for a menu (pause, dialog) over a stalled race.
// A hitch alone is no pause: its keys would leak into PLAY (hitch-input-leak).
inline constexpr uint32_t INPUT_GATE_PAUSE_MS = 250;

// Ages in ms since the last Supreme::Cycle and UI_Menu::Execute.
inline bool IsMenuPause(uint32_t cycle_age_ms, bool menu_seen, uint64_t menu_age_ms) {
    return cycle_age_ms > INPUT_GATE_PAUSE_MS && menu_seen && menu_age_ms <= INPUT_GATE_PAUSE_MS;
}

// TasMode values, without pulling shared_state.hpp into tests.
inline constexpr uint32_t INPUT_GATE_MODE_OFF = 0;
inline constexpr uint32_t INPUT_GATE_MODE_REC = 1;
inline constexpr uint32_t INPUT_GATE_MODE_PLAY = 2;

// The Windows virtual keys of the six recorded keys (window messages carry
// the neutral Ctrl/Shift codes).
inline bool IsTasVirtualKey(uint32_t vk) {
    switch (vk) {
        case 0x25: case 0x26: case 0x27: case 0x28:  // VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN
        case 0x10: case 0x11:                         // VK_SHIFT, VK_CONTROL
            return true;
        default:
            return false;
    }
}

inline bool ShouldBlockRealInput(const InputGateInputs& in) {
    if (in.injecting) return false;
    // ESC is the abort key and drives the pause menu.
    if (in.is_escape) return false;
    // A release adds no input, and a key a take cannot hold may have been
    // pressed while it was let through (a menu, OFF): blocking its release
    // would leave it down.
    if (in.is_release && !in.is_tas_key) return false;

    // A CONT blocks even while paused: the F5 reload stalls the cycle, and a
    // key held into the restart can move the spawn.
    if (in.cont_suppress) return true;

    // PLAY blocks, lifted while paused so menus stay usable.
    if (in.mode == INPUT_GATE_MODE_PLAY && !in.game_paused) return true;
    // REC lets the recorded keys through and records what the game applied;
    // any other key could change the run without being in the take.
    if (in.mode == INPUT_GATE_MODE_REC && !in.is_tas_key && !in.game_paused) return true;

    return false;
}
