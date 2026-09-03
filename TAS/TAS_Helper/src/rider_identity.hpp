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

static bool TryReadU32(uint32_t addr, uint32_t* v) {
    return SafeCopy(addr, v, sizeof *v);
}

// MSVC6 std::string at `obj`: {allocator, char* ptr, size, capacity}. The
// header is copied in ONE guarded read and bounds-checked without arithmetic
// on the length (riderparse::StringHeaderUsable): a dead object can report
// any size, and `len + 1` wrapped for 0xFFFFFFFF.
static bool ReadStdString(uint32_t obj, char* out, uint32_t cap) {
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

// Is `setup` the game-setup object for a rider called `name`?
static bool SetupValid(uint32_t setup, const char* name) {
    if (setup < 0x10000) return false;
    char s[32];
    if (!ReadStdString(setup + GameAddresses::SETUP_CHARACTER_STRING, s, sizeof s)) return false;
    if (!riderparse::EqualsIgnoreCase(s, name)) return false;
    uint32_t st = 0;
    if (!TryReadU32(setup + GameAddresses::SETUP_STANCE, &st) || st > 1) return false;
    // The controller string is a structural check only (a readable, printable
    // MSVC6 string at +0x290). It is deliberately NOT compared to "Keyboard":
    // a gamepad player's setup object must still be found.
    if (!ReadStdString(setup + GameAddresses::SETUP_CONTROLLER_STRING, s, sizeof s)) return false;
    return true;
}

// Heap scan for the setup object (see the header comment). Cheap dword
// filters first, the string reads only on survivors. Returns 0 if not found.
static uint32_t FindSetup(const char* name, uint64_t* bytesScanned, uint32_t* regionsScanned) {
    MEMORY_BASIC_INFORMATION mbi{};
    *bytesScanned = 0;
    *regionsScanned = 0;
    uintptr_t addr = 0x10000;
    while (addr < 0x7FFF0000u && VirtualQuery((void*)addr, &mbi, sizeof mbi) == sizeof mbi) {
        const uintptr_t base = (uintptr_t)mbi.BaseAddress;
        const uintptr_t size = mbi.RegionSize;
        const bool heapRw = mbi.State == MEM_COMMIT && mbi.Type == MEM_PRIVATE &&
                            (mbi.Protect & PAGE_GUARD) == 0 &&
                            (mbi.Protect == PAGE_READWRITE || mbi.Protect == PAGE_EXECUTE_READWRITE);
        if (heapRw && size >= 0x300 && size <= 64u * 1024 * 1024) {
            *bytesScanned += size;
            (*regionsScanned)++;
            __try {
                const uint8_t* p = (const uint8_t*)base;
                // Every field read below is < 0x2A0 past `off`; size >= 0x300 here.
                for (uintptr_t off = 0; off + 0x2A0 <= size; off += 4) {
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
            LARGE_INTEGER f, t0, t1;
            QueryPerformanceFrequency(&f);
            QueryPerformanceCounter(&t0);
            uint64_t bytes = 0;
            uint32_t regions = 0;
            s_setup = FindSetup(name, &bytes, &regions);
            QueryPerformanceCounter(&t1);
            const double ms = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)f.QuadPart;
            static uint32_t s_scans = 0;
            s_scans++;
            Log(std::format("Rider: game-setup scan #{} {} - {:.1f} ms over {} regions / {:.1f} MB (rider {}, worker thread)",
                            s_scans, s_setup ? std::format("found {:#010x}", s_setup) : "not found", ms, regions,
                            (double)bytes / (1024.0 * 1024.0), name));
        }
    }
    uint32_t stance = 0xFFFFFFFFu;
    if (s_setup && !(TryReadU32(s_setup + GameAddresses::SETUP_STANCE, &stance) && stance <= 1)) {
        stance = 0xFFFFFFFFu;   // a faulted read is UNKNOWN, never "regular"
    }

    // The pair is published under rider_seq so a reader never pairs a new
    // character with the previous stance (codex review 2026-09-03: a REC armed
    // in that window would stamp the mixed identity into the file). Bumped
    // only when the value changes, so steady state costs readers nothing.
    static bool s_seqChecked = false;
    if (!s_seqChecked) {
        s_seqChecked = true;
        // Shared memory survives reinjection: a previous DLL killed mid-write
        // leaves the sequence ODD and every reader rejecting forever.
        if (s->rider_seq & 1) InterlockedIncrement((volatile LONG*)&s->rider_seq);
    }
    if (s->rider_character != character || s->rider_stance != stance) {
        InterlockedIncrement((volatile LONG*)&s->rider_seq);   // odd: writing
        s->rider_character = character;
        s->rider_stance = stance;
        InterlockedIncrement((volatile LONG*)&s->rider_seq);   // even: stable
    }

    if (character != lastCharacter || stance != lastStance) {
        lastCharacter = character;
        lastStance = stance;
        Log(std::format("Rider: {} (id {}), stance {} ({}) - setup {:#010x}", name, character, stance,
                        stance == 0 ? "regular" : stance == 1 ? "goofy" : "unknown", s_setup));
    }
}

} // namespace rider
