#pragma once
#include "stdafx.h"
#include "game_addresses.hpp"
#include "setup_config_parse.hpp"
#include "safe_read.hpp"
#include "log.hpp"
#include <format>

// The menu selection (area, difficulty, character, stance, controller) from the
// executable's game-config object at [[Supreme.exe+0x889C4]+0x14]. Field
// offsets are in setup_config_parse.hpp. Unlike the level path, it reads
// "Village"/"Hard" for Village Hard.
namespace gamesetup {

using Setup = setupconfig::Values;

// Returns false when the chain or any string field is unreadable. The stance
// is UNKNOWN on a failed read, never 0 (regular). Logs each resolved/unresolved
// transition so a broken chain shows up on the first race.
inline bool Read(Setup* out) {
    const HMODULE exe = GetModuleHandleA(nullptr);
    auto reader = SafeCopy;
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
