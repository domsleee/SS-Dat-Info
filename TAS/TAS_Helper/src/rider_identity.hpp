#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"
#include <format>

// Rider awareness: who is on the board, and in which stance.
//
// The physics depend on the character (a Keith recording does not line up
// when Vincent rides it) and on the stance, so the DLL publishes both and
// tas_ui stamps recordings / history entries with them and warns when a
// replay's stamp differs from the live one - the same shape as the renderer /
// x87-precision stamp. (The board does not affect the physics.)
//
// Sources:
//   character - the human Player's Player_Config name ([player+0x48] ->
//               std::string at +0x48, "Vincent"; the loadout folder at
//               [[player+0x20]+0x10] as the fallback).
//   stance    - the menu's game-setup object, which the game builds every
//               rider from when a level is entered. Nothing static points at
//               it reliably (a Main_Menu.dll static that did once pointed
//               elsewhere on the next launch), so it is FOUND by layout: a
//               heap scan for the MSVC6 std::string naming the live rider at
//               +0x1D0 with the stance dword (0/1) at +0x220 and the
//               controller string ("Keyboard") at +0x290 (GameAddresses).
//               The hit is cached and re-validated on every refresh; the scan
//               itself runs at most every 2 s while nothing valid is cached.
//               Read-only: the stance cannot be switched in-process (see
//               GameAddresses), so a mismatch is reported, never "fixed".
//
// All reads are SEH-guarded from the level-scan worker (~10 Hz): the player
// set is rebuilt on every restart, so a read can land on a dead object; a
// failed or implausible read keeps the last published value.
namespace rider {

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

// MSVC6 std::string at `obj`: {allocator, char* ptr, size, capacity}.
static bool ReadStdString(uint32_t obj, char* out, uint32_t cap) {
    const uint32_t ptr = SafeU32(obj + GameAddresses::MSVC6_STRING_PTR);
    const uint32_t len = SafeU32(obj + GameAddresses::MSVC6_STRING_SIZE);
    if (!ptr || len == 0 || len + 1 > cap) return false;
    if (!SafeCopy(ptr, out, len)) return false;
    out[len] = 0;
    return riderparse::IsPrintableAscii(out, len);
}

// Is `setup` the game-setup object for a rider called `name`?
static bool SetupValid(uint32_t setup, const char* name) {
    if (setup < 0x10000) return false;
    char s[32];
    if (!ReadStdString(setup + GameAddresses::SETUP_CHARACTER_STRING, s, sizeof s)) return false;
    if (!riderparse::EqualsIgnoreCase(s, name)) return false;
    if (SafeU32(setup + GameAddresses::SETUP_STANCE) > 1) return false;
    if (!ReadStdString(setup + GameAddresses::SETUP_CONTROLLER_STRING, s, sizeof s)) return false;
    return true;
}

// Heap scan for the setup object (see the header comment). Cheap dword
// filters first, the string reads only on survivors. Returns 0 if not found.
static uint32_t FindSetup(const char* name) {
    MEMORY_BASIC_INFORMATION mbi{};
    uintptr_t addr = 0x10000;
    while (addr < 0x7FFF0000u && VirtualQuery((void*)addr, &mbi, sizeof mbi) == sizeof mbi) {
        const uintptr_t base = (uintptr_t)mbi.BaseAddress;
        const uintptr_t size = mbi.RegionSize;
        const bool heapRw = mbi.State == MEM_COMMIT && mbi.Type == MEM_PRIVATE &&
                            (mbi.Protect & PAGE_GUARD) == 0 &&
                            (mbi.Protect == PAGE_READWRITE || mbi.Protect == PAGE_EXECUTE_READWRITE);
        if (heapRw && size >= 0x300 && size <= 64u * 1024 * 1024) {
            __try {
                const uint8_t* p = (const uint8_t*)base;
                const uintptr_t end = size - 0x2A0;
                for (uintptr_t off = 0; off < end; off += 4) {
                    const uint32_t* d = (const uint32_t*)(p + off);
                    // +0x220 stance in {0,1}; +0x1D0 string: marker, ptr, len 1..31, cap 31;
                    // +0x290 controller string: marker, ptr, len 1..31, cap 31.
                    if (d[0x220 / 4] > 1) continue;
                    if (d[0x1D0 / 4] != 0x19 || d[0x1DC / 4] != 0x1F) continue;
                    const uint32_t nameLen = d[0x1D8 / 4];
                    if (nameLen == 0 || nameLen > 31) continue;
                    if (d[0x290 / 4] != 0x19 || d[0x29C / 4] != 0x1F) continue;
                    const uint32_t setup = (uint32_t)(base + off);
                    if (SetupValid(setup, name)) return setup;
                }
            } __except (EXCEPTION_EXECUTE_HANDLER) {
                // a page went away mid-scan: skip the region
            }
        }
        addr = base + size;
        if (size == 0) break;
    }
    return 0;
}

// Publish rider_character / rider_stance; log on change.
inline void Refresh(TasSharedState* s) {
    static uint32_t lastCharacter = 0xFFFFFFFFu;
    static uint32_t lastStance = 0xFFFFFFFFu;
    static uint32_t s_setup = 0;
    static ULONGLONG s_lastScanMs = 0;

    const uint32_t player = s->player_ptr;
    if (!player) return;
    const uint32_t loadout = SafeU32(player + GameAddresses::PLAYER_LOADOUT_OFFSET);
    if (!loadout) return;

    char name[32];
    const uint32_t config = SafeU32(player + GameAddresses::PLAYER_CONFIG_OFFSET);
    if (!(config && ReadStdString(config + GameAddresses::PLAYER_CONFIG_NAME_STRING, name, sizeof name)) &&
        !ReadStdString(loadout + GameAddresses::LOADOUT_FOLDER_STRING, name, sizeof name)) {
        return;
    }
    const uint32_t character = riderparse::CharacterFromName(name);

    // The setup object: cached while it still validates, re-found (rate
    // limited) otherwise.
    if (!SetupValid(s_setup, name)) {
        s_setup = 0;
        const ULONGLONG now = GetTickCount64();
        if (now - s_lastScanMs >= 2000) {
            s_lastScanMs = now;
            s_setup = FindSetup(name);
            if (s_setup) Log(std::format("Rider: game-setup object found at {:#010x} (rider {})", s_setup, name));
        }
    }
    const uint32_t stance = s_setup ? SafeU32(s_setup + GameAddresses::SETUP_STANCE) : 0xFFFFFFFFu;

    s->rider_character = character;
    s->rider_stance = stance;

    if (character != lastCharacter || stance != lastStance) {
        lastCharacter = character;
        lastStance = stance;
        Log(std::format("Rider: {} (id {}), stance {} ({}) - setup {:#010x}", name, character, stance,
                        stance == 0 ? "regular" : stance == 1 ? "goofy" : "unknown", s_setup));
    }
}

} // namespace rider
