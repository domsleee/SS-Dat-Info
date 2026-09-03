#pragma once
#include "stdafx.h"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"

// The game-setup object: one stable object that names the whole menu
// selection, reachable WITHOUT a heap scan.
//
// It hangs off the same root cave2 resolves for input injection
// ([SG+0x1D5450], a Supreme_Keyboard), but in the NEXT slot: [root+0x540]
// (GameAddresses::SETUP_OBJ_OFFSET). [root+0x530] is the Cetsup::Win32_Keyboard
// cave2 injects into - a different object whose bytes at these offsets are
// input state; reading the setup through it fails every string check (that
// was the bug from 2026-09-03 to 09-04: stance and level unknown on every
// race). The setup object carries the menu's selection as MSVC6 std::strings:
//   +0x190 area        ("Forest" / "Alpine" / "Village" / "Practice")
//   +0x1B0 difficulty  ("Easy" / "Medium" / "Hard")
//   +0x1D0 character    ("Keith", ...)
//   +0x220 stance dword (0 = regular, 1 = goofy)
//   +0x290 controller  ("Keyboard")
// (Verified live 2026-09-04 by a layout probe: root+0x540 is the only root
// slot holding the object whose strings read Forest/Hard/Keith/Keyboard, and
// it reads "Village"/"Hard" for Village Hard where the shared path asset reads
// ".../village/Tracks/easy" - so it also settles the one level pair the path
// string cannot.)
//
// This replaces two heap scans (rider_identity's setup search and level_scan's
// difficulty tally) with three pointer reads. Every access is SEH-guarded: the
// object is rebuilt across level loads, so a read can land mid-teardown.
namespace gamesetup {

struct Setup {
    bool valid = false;
    char area[32] = {};
    char difficulty[32] = {};
    char character[32] = {};
    uint32_t stance = 0xFFFFFFFFu;  // 0/1, or 0xFFFFFFFF when the read faulted
};

static bool SafeCopy(uint32_t src, void* dst, uint32_t n) {
    if (src < 0x10000) return false;
    __try {
        memcpy(dst, (const void*)(uintptr_t)src, n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

static uint32_t SafeU32(uint32_t addr) {
    uint32_t v = 0;
    return SafeCopy(addr, &v, sizeof v) ? v : 0;
}

static bool TryU32(uint32_t addr, uint32_t* v) { return SafeCopy(addr, v, sizeof *v); }

// MSVC6 std::string at `obj`: {allocator, char* ptr, size, capacity}. The
// header is copied in ONE guarded read and bounds-checked without arithmetic on
// the length (see riderparse::StringHeaderUsable).
static bool ReadStr(uint32_t obj, char* out, uint32_t cap) {
    uint32_t hdr[4] = {};
    if (!SafeCopy(obj, hdr, sizeof hdr)) return false;
    const uint32_t ptr = hdr[GameAddresses::MSVC6_STRING_PTR / 4];
    const uint32_t len = hdr[GameAddresses::MSVC6_STRING_SIZE / 4];
    const uint32_t capacity = hdr[3];
    if (!riderparse::StringHeaderUsable(ptr, len, capacity, cap)) return false;
    if (!SafeCopy(ptr, out, len)) return false;
    out[len] = 0;
    return riderparse::IsPrintableAscii(out, len);
}

// Resolve [[playerBaseAddr]+0x540] and read the fields. Returns false when the
// chain or any of area/difficulty/character/controller is unreadable - the
// safe direction (the caller then keeps its last value / stays unresolved).
// The controller string is a structural proof this is the setup object and not
// some other allocation transiently sitting at [root+0x530]. The stance is
// UNKNOWN on a faulted read, never 0 = "regular".
inline bool Read(uint32_t playerBaseAddr, Setup* out) {
    *out = Setup{};
    const uint32_t root = SafeU32(playerBaseAddr);
    if (root < 0x10000) return false;
    const uint32_t kb = SafeU32(root + GameAddresses::SETUP_OBJ_OFFSET);
    if (kb < 0x10000) return false;

    char ctrl[32];
    if (!ReadStr(kb + GameAddresses::SETUP_CONTROLLER_STRING, ctrl, sizeof ctrl)) return false;
    if (!ReadStr(kb + GameAddresses::SETUP_AREA_STRING, out->area, sizeof out->area)) return false;
    if (!ReadStr(kb + GameAddresses::SETUP_DIFFICULTY_STRING, out->difficulty, sizeof out->difficulty)) return false;
    if (!ReadStr(kb + GameAddresses::SETUP_CHARACTER_STRING, out->character, sizeof out->character)) return false;

    uint32_t st = 0xFFFFFFFFu;
    if (TryU32(kb + GameAddresses::SETUP_STANCE, &st) && st <= 1) out->stance = st;
    out->valid = true;
    return true;
}

}  // namespace gamesetup
