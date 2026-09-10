#pragma once
#include <cstdint>

// Pure decision logic for the srConfig unlink guard (srconfig_guard.hpp),
// kept free of Windows / hook context so src/tests/test_srconfig_guard.cpp
// can exercise it.
//
// sr.dll's unexported hash-node unlink at sr+0x13550 (the helper right after
// srConfig::append; callers srConfig::removeAll at level teardown and
// srConfig::set when a high time replaces a key) trusts the node it is
// handed. Every one of the 11 sr.dll crashes in the 30 days before
// 2026-09-09 faulted inside it with node == 3: the list was already corrupt
// (re-entrant F5 restarts tearing the level down over a rebuild in flight,
// see restart_release.hpp), and the game died at save / exit, eating the run.
//
// A node is unlinked only when it is genuinely linked: readable, and its
// neighbours point BACK at it (reciprocity). Readability alone is not
// enough - a stale node usually still points into mapped heap - and a
// corrupt node that fails the test is skipped (one leaked node) instead of
// letting the write scribble on whatever now lives there.
//
// Node layout (from the disassembly): +0 key, +8 prev, +0xC next.

// Reads a pointer-sized word at `address`; returns false when the read
// faults or the address is not a plausible heap address.
using SrConfigReadPtr = bool (*)(uintptr_t address, uintptr_t* out);

inline constexpr uintptr_t SRCONFIG_NODE_PREV = 0x8;
inline constexpr uintptr_t SRCONFIG_NODE_NEXT = 0xC;
inline constexpr uintptr_t SRCONFIG_MIN_HEAP_ADDRESS = 0x10000;

inline bool SrConfigNodeLooksSane(uintptr_t node, SrConfigReadPtr read) {
    if (node < SRCONFIG_MIN_HEAP_ADDRESS) return false;
    uintptr_t prev = 0, next = 0;
    if (!read(node + SRCONFIG_NODE_PREV, &prev)) return false;
    if (!read(node + SRCONFIG_NODE_NEXT, &next)) return false;
    if (prev) {
        if (prev < SRCONFIG_MIN_HEAP_ADDRESS) return false;
        uintptr_t prevNext = 0;
        if (!read(prev + SRCONFIG_NODE_NEXT, &prevNext)) return false;
        if (prevNext != node) return false;  // reciprocity broken
    }
    if (next) {
        if (next < SRCONFIG_MIN_HEAP_ADDRESS) return false;
        uintptr_t nextPrev = 0;
        if (!read(next + SRCONFIG_NODE_PREV, &nextPrev)) return false;
        if (nextPrev != node) return false;  // reciprocity broken
    }
    return true;
}
