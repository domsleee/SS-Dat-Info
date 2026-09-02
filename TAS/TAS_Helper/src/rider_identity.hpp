#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"
#include <format>

// Rider awareness: who is on the board.
//
// The physics depend on the character (a Keith recording does not line up
// when Vincent rides it) and on the stance, so the DLL publishes both and
// tas_ui stamps recordings / history entries with them and warns when a
// replay's stamp differs from the live one - the same shape as the renderer /
// x87-precision stamp. (The board does not affect the physics.)
//
// Source: the human Player's Player_Config name ([player+0x48] -> std::string
// at +0x48, "Vincent"; the loadout folder at [[player+0x20]+0x10] as the
// fallback) and the loadout's stance word ([[player+0x20]+0x20] & 0xFFFF);
// offsets in GameAddresses. Read by the level-scan worker (~10 Hz) with
// SEH-guarded copies - the player set is rebuilt on every restart, so a read
// can land on a dead object; a failed or implausible read keeps the last
// published value.
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

// Publish rider_character / rider_stance; log on change.
inline void Refresh(TasSharedState* s) {
    static uint32_t lastCharacter = 0xFFFFFFFFu;
    static uint32_t lastStance = 0xFFFFFFFFu;

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
    const uint32_t stance = SafeU32(loadout + GameAddresses::LOADOUT_STANCE_WORD) & 0xFFFFu;

    s->rider_character = character;
    s->rider_stance = stance;

    if (character != lastCharacter || stance != lastStance) {
        lastCharacter = character;
        lastStance = stance;
        Log(std::format("Rider: {} (id {}), stance word {} - loadout {:#010x}", name, character, stance, loadout));
    }
}

} // namespace rider
