#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"
#include "setup_object.hpp"
#include <format>

// Rider awareness: who is on the board, and in which stance.
//
// The physics depend on the character (a Keith recording does not line up
// when Vincent rides it) and on the stance, so the DLL publishes both and
// tas_ui stamps recordings / history entries with them and warns when a
// replay's stamp differs from the live one - the same shape as the renderer /
// x87-precision stamp. (The board does not affect the physics.)
//
// Sources (no heap scan):
//   character - the human Player's Player_Config name ([player+0x48] ->
//               std::string at +0x48, "Vincent"; the loadout folder at
//               [[player+0x20]+0x10] as the fallback).
//   stance    - the game-setup object read through the stable executable
//               config pointer chain (setup_object.hpp), the value the game
//               builds every rider from when a level is entered. Validated by
//               comparing the setup's character string to the live rider, so a
//               stale or mid-menu object is rejected rather than believed.
//               Read-only: the stance cannot be switched in-process, so a
//               mismatch is reported, never "fixed".
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

// MSVC6 std::string at `obj`: {allocator, char* ptr, size, capacity}. Header
// read in one guarded copy and bounds-checked without arithmetic on the length
// (riderparse::StringHeaderUsable): a dead object can report any size.
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

// Publish rider_character / rider_stance; log on change.
//
inline void Refresh(TasSharedState* s) {
    static uint32_t lastCharacter = 0xFFFFFFFFu;
    static uint32_t lastStance = 0xFFFFFFFFu;

    const uint32_t player = s->player_ptr;
    if (!player) return;
    const uint32_t loadout = SafeU32(player + GameAddresses::PLAYER_LOADOUT_OFFSET);
    if (!loadout) return;

    // Character from the LIVE Player (the truth on the board right now).
    char name[32];
    const uint32_t config = SafeU32(player + GameAddresses::PLAYER_CONFIG_OFFSET);
    if (!(config && ReadStdString(config + GameAddresses::PLAYER_CONFIG_NAME_STRING, name, sizeof name)) &&
        !ReadStdString(loadout + GameAddresses::LOADOUT_FOLDER_STRING, name, sizeof name)) {
        return;
    }
    const uint32_t character = riderparse::CharacterFromName(name);

    // Stance from the game-setup object via the stable chain. Trust it only
    // when its own character string matches the live rider - otherwise it is
    // stale or mid-menu-change, and the stance is unknown (never assumed
    // regular).
    uint32_t stance = riderparse::STANCE_UNKNOWN;
    gamesetup::Setup setup;
    if (gamesetup::Read(&setup)) {
        stance = riderparse::StanceForRider(setup.stance, setup.character, name);
    }

    // The pair is published under rider_seq so a reader never pairs a new
    // character with the previous stance (a REC armed in that window would
    // stamp the mixed identity into the file). Bumped only when the value
    // changes, so steady state costs readers nothing.
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
        Log(std::format("Rider: {} (id {}), stance {} ({})", name, character, stance,
                        stance == 0 ? "regular" : stance == 1 ? "goofy" : "unknown"));
    }
}

}  // namespace rider
