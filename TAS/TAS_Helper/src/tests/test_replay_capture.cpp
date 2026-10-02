// Unit tests for the replay-recorder capture policy (replay_capture_policy.hpp).
// Pure logic, no Windows/hook deps; built and run by TAS/tools/test_dll_hidden.ps1.
//
// Two live regressions pin the policy:
//   ghost_restart_recreates_recorder_mid_play - with Time Attack ghosts an F5
//     restart rebuilds the player set; the human's recorder/player come back at
//     NEW addresses while a PLAY is active, so the cached pointer must
//     follow the re-created human recorder instead of reading a dead object.
//   transient_pushers_never_hijack - around a restart, ghost / AI / garbage
//     recorders push on some frames and must never be adopted.
// Both follow from the identity check: the owner is a plain `Player` still
// linked to the recorder.

#include "../replay_capture_policy.hpp"
#include "../replay_identity.hpp"
#include "check.hpp"
#include <cstdio>
#include <map>

int main() {
    std::printf("replay_capture tests:\n");
    constexpr bool HUMAN = true, OTHER = false;
    // Addresses as observed in-game.
    constexpr uint32_t HUMAN_A = 0x0AEF1BA8, HUMAN_B = 0x0AF16450, GHOST = 0x0C4610C0,
                       GARBAGE_OWNER_REC = 0x0C460630;

    {
        // same_address_reuse_is_caught: F5 frees the human's recorder and the
        // allocator hands the SAME address to a ghost. An address-change gate
        // cannot see that; revalidation on every push of the cached address does.
        ReplayCaptureState st;
        check(ReplayCaptureAdopt(HUMAN_A, HUMAN, st) && st.cached == HUMAN_A, "reuse: human adopted");
        check(!ReplayCaptureRevalidate(true, st) && st.cached == HUMAN_A,
              "reuse: still human on the next push - nothing happens");
        check(ReplayCaptureRevalidate(false, st) && st.cached == 0,
              "reuse: the same address now owned by a ghost is DROPPED");
        check(!ReplayCaptureRevalidate(false, st),
              "reuse: nothing cached - a further non-human push is a no-op");
        check(!ReplayCaptureAdopt(HUMAN_A, OTHER, st) && st.cached == 0,
              "reuse: the ghost at the old address is never adopted");
        check(ReplayCaptureAdopt(HUMAN_A, HUMAN, st) && st.cached == HUMAN_A,
              "reuse: the human re-created at that address is adopted again (mid-run)");
    }

    {
        ReplayCaptureState st;
        check(ReplayCaptureAdopt(HUMAN_A, HUMAN, st) && st.cached == HUMAN_A,
              "idle: the human's recorder is adopted on its first push");
        check(!ReplayCaptureAdopt(HUMAN_A, HUMAN, st), "same recorder again is not a change");
        check(st.rejected == 0, "clean counter");
    }
    {
        // transient_pushers_never_hijack: however often a non-human recorder
        // pushes, idle or mid-run, the human stays followed.
        ReplayCaptureState st;
        ReplayCaptureAdopt(HUMAN_A, HUMAN, st);
        bool any = false;
        for (int i = 0; i < 50; i++) {
            any = ReplayCaptureAdopt(GHOST, OTHER, st) || any;
            any = ReplayCaptureAdopt(GARBAGE_OWNER_REC, OTHER, st) || any;
        }
        check(!any && st.cached == HUMAN_A, "transient_pushers_never_hijack: ghost / garbage pushers ignored");
        check(st.rejected == 100, "each ignored pusher is counted (diagnostic)");
    }
    {
        // ghost_restart_recreates_recorder_mid_play: REC on HUMAN_A, the PLAY
        // restart rebuilds the player set, HUMAN_B is the new human recorder.
        ReplayCaptureState st;
        ReplayCaptureAdopt(HUMAN_A, HUMAN, st);
        check(ReplayCaptureAdopt(HUMAN_B, HUMAN, st) && st.cached == HUMAN_B,
              "ghost_restart_recreates_recorder_mid_play: re-created human recorder adopted mid-run");
        check(!ReplayCaptureAdopt(HUMAN_B, HUMAN, st), "steady state: no churn while it stays put");
    }
    {
        ReplayCaptureState st;
        check(!ReplayCaptureAdopt(GHOST, OTHER, st) && st.cached == 0,
              "nothing cached and a non-human pusher: still nothing (never follow a ghost)");
        check(ReplayCaptureAdopt(HUMAN_A, HUMAN, st) && st.cached == HUMAN_A,
              "first human recorder mid-run is adopted");
        check(!ReplayCaptureAdopt(0, HUMAN, st) && st.cached == HUMAN_A,
              "a null ECX never replaces a live recorder");
    }

    {
        // Owner classification against a fake process image. Layout as in the
        // live game: recorder+0x84 -> owner, owner+0x14C -> recorder,
        // [owner] = vtable. Class identity has no offset to get wrong.
        std::map<uint32_t, uint32_t> mem;
        auto read = [&](uint32_t a) -> uint32_t {
            auto it = mem.find(a);
            return it == mem.end() ? 0u : it->second;
        };
        constexpr uint32_t PLAYER_VT = 0x00E39E10, GHOST_VT = 0x00E39B74, AI_VT = 0x00E39C40;
        constexpr uint32_t HUMAN = 0x0D873990, HUMAN_REC = 0x0AF103D8;
        constexpr uint32_t GHOST = 0x0D4648C0, GHOST_REC = 0x0C4610C0;
        constexpr uint32_t AI = 0x0D500000, AI_REC = 0x0C500000;
        constexpr uint32_t DEAD_REC = 0x0AEF1BA8;  // human's recorder from BEFORE a ghost restart
        mem[HUMAN] = PLAYER_VT;      mem[HUMAN + 0x14C] = HUMAN_REC;  mem[HUMAN_REC + 0x84] = HUMAN;
        mem[GHOST] = GHOST_VT;    mem[GHOST + 0x14C] = GHOST_REC;  mem[GHOST_REC + 0x84] = GHOST;
        mem[AI] = AI_VT;          mem[AI + 0x14C] = AI_REC;        mem[AI_REC + 0x84] = AI;
        mem[DEAD_REC + 0x84] = HUMAN;  // still names the human, who has moved on to HUMAN_REC
        ReplayIdentityTrace t{};

        check(ClassifyRecorderOwner(HUMAN_REC, PLAYER_VT, read, &t) == OWNER_HUMAN,
              "identity: the Player's linked recorder is the human's");
        check(t.owner == HUMAN && t.owner_vtable == PLAYER_VT && t.owner_recorder == HUMAN_REC,
              "identity: trace carries owner / vtable / back-link");
        check(ClassifyRecorderOwner(GHOST_REC, PLAYER_VT, read, &t) == OWNER_OTHER,
              "identity: a Ghost_Player's recorder is 'other' (never adopted)");
        check(ClassifyRecorderOwner(AI_REC, PLAYER_VT, read, &t) == OWNER_OTHER,
              "identity: an AI_Player's recorder is 'other' (never adopted)");
        check(ClassifyRecorderOwner(DEAD_REC, PLAYER_VT, read, &t) == OWNER_UNLINKED,
              "identity: the human's PREVIOUS recorder is unlinked once the player set was rebuilt");
        check(ClassifyRecorderOwner(0x0C460630, PLAYER_VT, read, &t) == OWNER_UNREADABLE,
              "identity: a pusher with an unreadable owner is rejected");
        mem[0x0C460630 + 0x84] = 2;  // the garbage-owner pusher seen around restarts
        check(ClassifyRecorderOwner(0x0C460630, PLAYER_VT, read, &t) == OWNER_UNREADABLE && t.owner == 2,
              "identity: owner == 2 (garbage) is rejected, traced");
        check(ClassifyRecorderOwner(0, PLAYER_VT, read, &t) == OWNER_UNREADABLE,
              "identity: a null ECX is rejected");
        check(ClassifyRecorderOwner(HUMAN_REC, 0, read, &t) == OWNER_OTHER,
              "identity: without a resolved Player vtable nothing is ever human");
        check(ReplayOwnerKindName(OWNER_HUMAN)[0] == 'h' && ReplayOwnerKindName(OWNER_OTHER)[0] == 'o',
              "identity: kind names for the ring log");
    }

    return FinishTests();
}
