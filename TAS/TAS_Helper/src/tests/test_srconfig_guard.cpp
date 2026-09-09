// Unit tests for the srConfig unlink guard policy (srconfig_guard_policy.hpp).
// Pure logic, no Windows/hook deps - compile + run standalone:
//   just test_dll      (from repo root)
//
// The headline case is `node_3_is_skipped`: every sr.dll crash dump from the
// 30 days before 2026-09-09 had the unlink handed node == 3. The guard must
// refuse it without touching memory; a genuinely linked node must pass so no
// real removal is ever leaked.

#include "../srconfig_guard_policy.hpp"
#include "check.hpp"
#include <cstdio>
#include <cstring>

// A fake 32-bit heap: nodes at fixed addresses, each 0x14 bytes.
static uintptr_t g_mem[0x100];  // covers addresses 0x100000 .. 0x100400
static const uintptr_t BASE = 0x100000;
static bool g_faultOn = false;

static bool readPtr(uintptr_t address, uintptr_t* out) {
    if (address < BASE || address + sizeof(uintptr_t) > BASE + sizeof g_mem) return false;
    if (g_faultOn) return false;
    *out = g_mem[(address - BASE) / sizeof(uintptr_t)];
    return true;
}

static void put(uintptr_t address, uintptr_t value) { g_mem[(address - BASE) / sizeof(uintptr_t)] = value; }
static uintptr_t node(int i) { return BASE + 0x40 * (i + 1); }

int main() {
    std::printf("srconfig_guard tests:\n");
    std::memset(g_mem, 0, sizeof g_mem);
    const uintptr_t a = node(0), b = node(1), c = node(2);
    // a <-> b <-> c, doubly linked
    put(a + SRCONFIG_NODE_NEXT, b);
    put(b + SRCONFIG_NODE_PREV, a);
    put(b + SRCONFIG_NODE_NEXT, c);
    put(c + SRCONFIG_NODE_PREV, b);

    check(!SrConfigNodeLooksSane(3, readPtr), "node_3_is_skipped (every field crash)");
    check(!SrConfigNodeLooksSane(0, readPtr), "null node is skipped");
    check(SrConfigNodeLooksSane(b, readPtr), "a linked middle node passes");
    check(SrConfigNodeLooksSane(a, readPtr), "the head (prev == 0) passes");
    check(SrConfigNodeLooksSane(c, readPtr), "the tail (next == 0) passes");

    // Stale node: readable, but its neighbours no longer point at it.
    const uintptr_t stale = node(3);
    put(stale + SRCONFIG_NODE_PREV, a);
    put(stale + SRCONFIG_NODE_NEXT, b);
    check(!SrConfigNodeLooksSane(stale, readPtr), "a readable node its neighbours disown is skipped");

    // Neighbour pointer into unmapped memory.
    const uintptr_t wild = node(4);
    put(wild + SRCONFIG_NODE_PREV, 0x7fff0000);
    check(!SrConfigNodeLooksSane(wild, readPtr), "an unreadable prev is skipped");
    put(wild + SRCONFIG_NODE_PREV, 0x8);  // below the heap floor
    check(!SrConfigNodeLooksSane(wild, readPtr), "a tiny prev pointer is skipped");

    // The node itself faults.
    g_faultOn = true;
    check(!SrConfigNodeLooksSane(b, readPtr), "a node that faults on read is skipped");
    g_faultOn = false;

    return FinishTests();
}
