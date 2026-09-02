#pragma once
#include <cstdint>

// Who owns a replay recorder? (pure logic, unit-tested in
// tests/test_replay_capture.cpp)
//
// Rider classes in Supreme_Game.dll (RTTI):
//   Player        - the rider the keyboard drives in a race (the base class
//                   itself; this is the one the TAS records)
//   Ghost_Player  - Time Attack TOP5 ghosts and the guide rider
//   AI_Player     - computer riders
//   Net_Player    - network riders
// A recorder links to its owner at +0x84 and the owner links back to its
// recorder at +0x14C (the player update pushes [player+0x14C] every tick).
//
// So a recorder is the human's iff its owner's vtable is Player's AND the
// owner's recorder slot still points at it. The second half rejects a dead
// recorder whose owner has since been rebuilt (ghost restarts re-create the
// player set) and any pusher with a garbage owner.
//
// History: the first identity test ([[player+0x1B8]+0x590] == keyboard object,
// 2026-09-02 morning) was a heap-adjacency artifact - +0x1B8 is the player's
// Player_Event_Interface (a Basic_Event in Time Attack) and the keyboard
// pointer sat 0x590 past a neighbouring allocation. It never matched in a
// fresh process, so the DLL adopted nothing and recorded all-zero coordinates
// (which a zero-vs-zero judge then "accepted"). A class check has no offset
// to get wrong, and both vtables are validated at Resolve.
struct ReplayIdentityEnv {
    uint32_t player_vtable = 0;  // live address of the Player vtable (SG + RVA)
    uint32_t ghost_vtable = 0;   // live address of the Ghost_Player vtable (diagnostic only)
};

struct ReplayIdentityTrace {
    uint32_t owner = 0;           // [recorder + 0x84]
    uint32_t owner_vtable = 0;    // [owner]
    uint32_t owner_recorder = 0;  // [owner + 0x14C]
};

enum ReplayOwnerKind : uint32_t {
    OWNER_UNREADABLE = 0,  // recorder / owner pointer not a usable address
    OWNER_UNLINKED = 1,    // owner does not (any more) point at this recorder
    OWNER_GHOST = 2,       // Ghost_Player
    OWNER_OTHER = 3,       // AI / net / unknown class
    OWNER_HUMAN = 4,       // Player, linked: the recorder to follow
};

static constexpr uint32_t REPLAY_IDENTITY_RECORDER_OWNER = 0x84;
static constexpr uint32_t REPLAY_IDENTITY_PLAYER_RECORDER = 0x14C;
static constexpr uint32_t REPLAY_IDENTITY_MIN_PTR = 0x10000;

// `read(addr)` returns the u32 at `addr` or 0 when unreadable.
template <class ReadU32>
inline ReplayOwnerKind ClassifyRecorderOwner(uint32_t recorder, const ReplayIdentityEnv& env,
                                             ReadU32 read, ReplayIdentityTrace* trace) {
    ReplayIdentityTrace t{};
    if (trace) *trace = t;
    if (recorder < REPLAY_IDENTITY_MIN_PTR) return OWNER_UNREADABLE;
    t.owner = read(recorder + REPLAY_IDENTITY_RECORDER_OWNER);
    if (trace) *trace = t;
    if (t.owner < REPLAY_IDENTITY_MIN_PTR) return OWNER_UNREADABLE;
    t.owner_vtable = read(t.owner);
    t.owner_recorder = read(t.owner + REPLAY_IDENTITY_PLAYER_RECORDER);
    if (trace) *trace = t;
    if (t.owner_recorder != recorder) return OWNER_UNLINKED;
    if (env.player_vtable && t.owner_vtable == env.player_vtable) return OWNER_HUMAN;
    if (env.ghost_vtable && t.owner_vtable == env.ghost_vtable) return OWNER_GHOST;
    return OWNER_OTHER;
}

inline const char* ReplayOwnerKindName(ReplayOwnerKind k) {
    switch (k) {
        case OWNER_UNREADABLE: return "unreadable";
        case OWNER_UNLINKED: return "unlinked";
        case OWNER_GHOST: return "ghost";
        case OWNER_OTHER: return "other";
        case OWNER_HUMAN: return "human";
    }
    return "?";
}
