#pragma once
#include "stdafx.h"
#include "game_addresses.hpp"
#include "setup_config_parse.hpp"
#include "log.hpp"
#include <format>

// The game-setup strings: the whole menu selection (area / track difficulty /
// character / stance / controller), read WITHOUT a heap scan.
//
// They are fields of the executable's game-config object, reached through the
// chain the stock game uses itself: [[Supreme.exe+0x889C4]+0x14]. The menu
// updates it as you choose:
//   +0x0B0 area        ("Forest" / "Alpine" / "Village" / "Practice")
//   +0x0D0 difficulty  ("Easy" / "Medium" / "Hard")
//   +0x0F0 character   ("Keith", ...)
//   +0x140 stance dword (0 = regular, 1 = goofy)
//   +0x1B0 controller  ("Keyboard" / "Mouse" / "Joystick")
// It reads "Village"/"Hard" for Village Hard where the shared path asset reads
// ".../village/Tracks/easy", so it also settles the one level pair the path
// string cannot.
//
// The earlier Main_Menu+0x6B9A4 anchor was a category error: it correctly
// points to Threedee_Engine::Engine, but these fields are not in that class.
// They lined up only through accidental heap adjacency on one launch. The EXE
// global above directly owns the config pointer; the pure reader and its unit
// tests validate the complete chain and field layout used here.
namespace gamesetup {

using Setup = setupconfig::Values;

static bool SafeCopy(uint32_t src, void* dst, uint32_t n) {
    if (src < 0x10000) return false;
    __try {
        memcpy(dst, (const void*)(uintptr_t)src, n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Read the menu selection. Returns false when the anchor or any of
// area/difficulty/character/controller is unreadable - the safe direction (the
// caller keeps its last value / stays unresolved). The controller string is a
// second, structural proof this is the right object. The stance is UNKNOWN on
// a faulted read, never 0 = "regular".
//
// Logs every resolved <-> unresolved transition: if this anchor ever rots the
// way the two before it did, the log says so on the first race instead of the
// level id and the rider stance going quietly blank.
inline bool Read(Setup* out) {
    const HMODULE exe = GetModuleHandleA(nullptr);
    auto reader = [](uint32_t address, void* destination, uint32_t size) {
        return SafeCopy(address, destination, size);
    };
    uint32_t config = 0;
    const bool ok = exe && setupconfig::Read(
        (uint32_t)(uintptr_t)exe, reader, out, &config);
    static int lastOk = -1;
    if ((int)ok != lastOk) {
        lastOk = (int)ok;
        if (ok)
            Log(std::format("Setup: config {:#x} ([[EXE+{:#x}]+{:#x}]) - area '{}' difficulty '{}' character '{}' stance {}",
                            config, setupconfig::MAIN_STATE_PTR_RVA,
                            setupconfig::CONFIG_PTR_OFFSET, out->area, out->difficulty,
                            out->character, out->stance == 0xFFFFFFFFu ? -1 : (int)out->stance));
        else
            Log(std::format("Setup: UNRESOLVED - [[EXE+{:#x}]+{:#x}] gave {:#x}; "
                            "the level id and the rider stance stay unknown",
                            setupconfig::MAIN_STATE_PTR_RVA,
                            setupconfig::CONFIG_PTR_OFFSET, config));
    }
    return ok;
}

}  // namespace gamesetup
