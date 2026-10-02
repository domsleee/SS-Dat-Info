#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../fpu_safe_hook.hpp"
#include "../shared_state.hpp"
#include "cycle_cave.hpp"

// The finish line, from the game's own Finish_Point (DESIGN.md "Race time").
// Its crossing callback (SG+0x52F90) calls the player's race timer finish
// (SG+0x7F9D0), which decides whether the finish counts (no checkpoint missed)
// and latches it once. The hook sits right after that decision:
//   SG+0x7FA33  test bl,bl ; mov byte [esi+0x11],1
//   bl = 1 valid, 0 a checkpoint was missed; esi = the timer, [esi+0x1C] its
//   owner Player, [esi+0x0C] the final time (float seconds).
// It runs inside Supreme::Cycle's physics; the cycle cave latched that tick's
// mode and index at its entry. Game thread only.
namespace finishline {

inline TasSharedState* g_state = nullptr;
inline uint32_t g_playerVtable = 0;
static SafetyHookMid g_hook{};

// Owner and final time of the finishing timer; false when unreadable.
static bool ReadFinish(uint32_t timer, uint32_t* owner, uint32_t* owner_vtable, uint32_t* time_bits) {
    if (timer < 0x10000) return false;
    __try {
        *owner = *(uint32_t*)(timer + 0x1C);
        *owner_vtable = *owner >= 0x10000 ? *(uint32_t*)(*owner) : 0;
        *time_bits = *(uint32_t*)(timer + 0x0C);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

static void OnFinish(SafetyHookContext& ctx) {
    auto* s = g_state;
    if (!s) return;
    uint32_t owner = 0, vtable = 0, bits = 0;
    if (!ReadFinish((uint32_t)ctx.esi, &owner, &vtable, &bits)) return;
    // The human rider only: ghosts and AI finish through the same code.
    if (owner != s->player_ptr || vtable != g_playerVtable) return;
    // Seqlock: odd while the fields change, so a reader never pairs one
    // finish's tick with another's time.
    InterlockedIncrement((volatile LONG*)&s->race_finish_seq);
    s->race_finish_tick = g_execTick;
    s->race_finish_mode = g_execMode;
    s->race_finish_valid = (ctx.ebx & 0xFF) ? 1u : 0u;
    s->race_finish_time_bits = bits;
    InterlockedIncrement((volatile LONG*)&s->race_finish_seq);
}

inline bool Install(GameAddresses& addr, TasSharedState* s) {
    if (!addr.sg) return false;
    auto sg = (uint8_t*)addr.sg;
    // SG+0x7FA1E..0x7FA38: no relocations; the hook site is at +0x15.
    static constexpr uint8_t kFinish[] = {
        0x84, 0xDB, 0x8B, 0x46, 0x0C, 0x89, 0x04, 0xBE, 0x75, 0x0A, 0x33, 0xC0, 0x89, 0x06,
        0x89, 0x46, 0x04, 0x89, 0x46, 0x08, 0x5F, 0x84, 0xDB, 0xC6, 0x46, 0x11, 0x01 };
    if (!GameAddresses::ValidateCode("Supreme_Game.dll+0x7FA1E (race timer finish)", sg + 0x7FA1E,
                                     kFinish)) {
        return false;
    }
    g_state = s;
    g_playerVtable = addr.player_vtable;
    g_hook = CreateMidHook<OnFinish>(sg + 0x7FA33);
    Log(std::format("Finish line: hooked {:p} (SG+0x7FA33) = {}", (void*)(sg + 0x7FA33), (bool)g_hook));
    if (!g_hook) g_state = nullptr;
    return (bool)g_hook;
}

inline void Uninstall() {
    g_hook = {};
    g_state = nullptr;
}

}  // namespace finishline
