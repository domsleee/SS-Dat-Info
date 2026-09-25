#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "rider_identity_parse.hpp"
#include "setup_object.hpp"
#include <format>

// Publishes the rider's character and stance, which both affect the physics;
// tas_ui stamps recordings with them and warns on a mismatch.
//   character: the Player_Config name ([player+0x48] -> std::string at +0x48),
//              falling back to the loadout folder [[player+0x20]+0x10].
//   stance:    the game-setup object (setup_object.hpp), trusted only when its
//              character matches the live rider. Read-only; cannot be switched in-process.
// The cycle cave calls Refresh on each race's first tick. A failed read keeps
// the last published value.
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

    uint32_t stance = riderparse::STANCE_UNKNOWN;
    gamesetup::Setup setup;
    if (gamesetup::Read(&setup)) {
        stance = riderparse::StanceForRider(setup.stance, setup.character, name);
    }

    // Published under rider_seq so a reader never pairs a new character with
    // the old stance. Bumped only on change.
    static bool s_seqChecked = false;
    if (!s_seqChecked) {
        s_seqChecked = true;
        // A previous instance killed mid-write can leave the sequence odd.
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
