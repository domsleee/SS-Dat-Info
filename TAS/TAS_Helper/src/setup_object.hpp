#pragma once
#include "stdafx.h"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"
#include "log.hpp"
#include <format>

// The game-setup strings: the whole menu selection (area / track difficulty /
// character / stance / controller), read WITHOUT a heap scan.
//
// They are fields of the 3D ENGINE object - HMG_3DE.dll's
// Threedee_Engine::Engine (RTTI-confirmed 2026-09-04) - which the menu updates
// as you choose:
//   +0x190 area        ("Forest" / "Alpine" / "Village" / "Practice")
//   +0x1B0 difficulty  ("Easy" / "Medium" / "Hard")
//   +0x1D0 character   ("Keith", ...)
//   +0x220 stance dword (0 = regular, 1 = goofy)
//   +0x290 controller  ("Keyboard")
// It reads "Village"/"Hard" for Village Hard where the shared path asset reads
// ".../village/Tracks/easy", so it also settles the one level pair the path
// string cannot.
//
// THE ANCHOR is the static Main_Menu.dll+0x6B9A4 - the menu's cached engine
// pointer, and the ONLY pointer to this object in any module image (a full
// image scan found no other). See GameAddresses for why the two previous
// keyboard-root offsets were both wrong, and note the shape of that mistake:
// an offset that happens to hold the right value once is not an anchor. This
// one is validated by the object's vtable on every read and logged when it
// resolves or stops resolving, so a silent failure is impossible.
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

// The engine object, or 0. Validated by class: its vtable must be exactly
// HMG_3DE.dll+0x25B9C (Threedee_Engine::Engine), so a stale or repurposed
// static can never be read as a setup.
inline uint32_t EngineObject() {
    const HMODULE mm = GetModuleHandleA("Main_Menu.dll");
    const HMODULE e3 = GetModuleHandleA("HMG_3DE.dll");
    if (!mm || !e3) return 0;
    const uint32_t obj = SafeU32((uint32_t)(uintptr_t)mm + GameAddresses::MAIN_MENU_ENGINE_PTR_RVA);
    if (obj < 0x10000) return 0;
    const uint32_t want = (uint32_t)(uintptr_t)e3 + GameAddresses::HMG3DE_ENGINE_VTABLE_RVA;
    return SafeU32(obj) == want ? obj : 0;
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
    *out = Setup{};
    const uint32_t obj = EngineObject();
    bool ok = obj != 0;
    if (ok) {
        char ctrl[32];
        ok = ReadStr(obj + GameAddresses::SETUP_CONTROLLER_STRING, ctrl, sizeof ctrl) &&
             ReadStr(obj + GameAddresses::SETUP_AREA_STRING, out->area, sizeof out->area) &&
             ReadStr(obj + GameAddresses::SETUP_DIFFICULTY_STRING, out->difficulty, sizeof out->difficulty) &&
             ReadStr(obj + GameAddresses::SETUP_CHARACTER_STRING, out->character, sizeof out->character);
        if (ok) {
            uint32_t st = 0xFFFFFFFFu;
            if (TryU32(obj + GameAddresses::SETUP_STANCE, &st) && st <= 1) out->stance = st;
            out->valid = true;
        }
    }
    static int lastOk = -1;
    if ((int)ok != lastOk) {
        lastOk = (int)ok;
        if (ok)
            Log(std::format("Setup: engine {:#x} (Main_Menu+{:#x}) - area '{}' difficulty '{}' character '{}' stance {}",
                            obj, GameAddresses::MAIN_MENU_ENGINE_PTR_RVA, out->area, out->difficulty,
                            out->character, out->stance == 0xFFFFFFFFu ? -1 : (int)out->stance));
        else
            Log(std::format("Setup: UNRESOLVED - Main_Menu+{:#x} gave {:#x} (needs vtable HMG_3DE+{:#x}); "
                            "the level id and the rider stance stay unknown",
                            GameAddresses::MAIN_MENU_ENGINE_PTR_RVA, obj,
                            GameAddresses::HMG3DE_ENGINE_VTABLE_RVA));
    }
    return ok;
}

}  // namespace gamesetup
