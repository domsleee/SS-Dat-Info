#pragma once
#include <cstdint>

// Pure decisions behind restart_release.hpp, kept free of Windows so
// src/tests/test_restart_release.cpp can exercise them.

// Minimum spacing between two F5 presses the game is allowed to see. One
// level restart takes ~30 ms and the key is taken back the moment the
// restart is observed, so anything arriving sooner than this is the
// keyboard's autorepeat (a human "holding F5 to be sure") or a bounce - the
// exact input that used to restart the level six or seven times per tap.
inline constexpr uint32_t F5_MIN_SPACING_MS = 500;

// Whether an F5 down at `nowMs` may reach the game given the last one it
// saw at `lastAcceptedMs` (0 = never). Wraparound-safe.
inline bool F5DownAccepted(uint32_t nowMs, uint32_t lastAcceptedMs, uint32_t spacingMs) {
    if (lastAcceptedMs == 0) return true;
    return (nowMs - lastAcceptedMs) >= spacingMs;
}

// An F5 up must be swallowed exactly when its down was, so the game never
// sees an unbalanced up: `downsSwallowed` counts swallowed downs whose up has
// not arrived yet.
inline bool F5UpSwallowed(uint32_t& downsSwallowed) {
    if (downsSwallowed == 0) return false;
    downsSwallowed--;
    return true;
}
