// Unit tests for the replay-recorder capture policy (replay_capture_policy.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)
// or:
//   cl /EHsc /std:c++17 /Fe:test_replay_capture.exe test_replay_capture.cpp && test_replay_capture.exe
//
// Regressions from 2026-09-02 (Forest Easy):
//   ghost_restart_recreates_recorder_mid_play - with Time Attack ghosts an F5
//     restart rebuilds the player set; the human's recorder/player come back at
//     NEW addresses while a judged PLAY is active. The original policy froze
//     the pointer for the whole run and kept reading a dead object: every PLAY
//     mismatched at gate+0 and rerolled forever.
//   transient_pushers_never_hijack - around a restart, ghost / AI / garbage
//     recorders push on some frames. "Follow the last pusher" adopted them and
//     FE without ghosts went from ~0 to 5-13 rerolls per PLAY.
// The identity check (the owner is a plain `Player` still linked to the recorder) resolves
// both; the pre-fix policies are kept here only so the regressions are
// demonstrable.

#include "../replay_capture_policy.hpp"
#include "../replay_identity.hpp"
#include <cstdio>
#include <map>

static int g_failures = 0;

static void check(bool cond, const char* name) {
    if (cond) {
        std::printf("  ok   %s\n", name);
    } else {
        std::printf("  FAIL %s\n", name);
        g_failures++;
    }
}

// Pre-fix policy #1: freeze while REC/PLAY.
static bool FreezeAdopt(bool mode_off, uint32_t incoming, uint32_t& cached) {
    if (mode_off || cached == 0) {
        cached = incoming;
        return true;
    }
    return false;
}
// Pre-fix policy #2: follow the last pusher.
static bool LastPusherAdopt(uint32_t incoming, uint32_t& cached) {
    if (incoming == 0 || incoming == cached) return false;
    cached = incoming;
    return true;
}

int main() {
    std::printf("replay_capture tests:\n");
    constexpr bool OFF = true, ACTIVE = false, HUMAN = true, OTHER = false;
    // Addresses as observed in-game.
    constexpr uint32_t HUMAN_A = 0x0AEF1BA8, HUMAN_B = 0x0AF16450, GHOST = 0x0C4610C0,
                       GARBAGE_OWNER_REC = 0x0C460630;

    {
        // same_address_reuse_is_caught (codex 2026-09-03): F5 frees the human's
        // recorder and the allocator hands the SAME address to a ghost. The
        // address-change gate never re-classified it; revalidation on every
        // push of the cached address does.
        ReplayCaptureState st;
        check(ReplayCaptureAdopt(OFF, HUMAN_A, HUMAN, st) && st.cached == HUMAN_A, "reuse: human adopted");
        check(!ReplayCaptureRevalidate(true, st) && st.cached == HUMAN_A && st.dropped == 0,
              "reuse: still human on the next push - nothing happens");
        check(ReplayCaptureRevalidate(false, st) && st.cached == 0 && st.dropped == 1,
              "reuse: the same address now owned by a ghost is DROPPED");
        check(!ReplayCaptureRevalidate(false, st) && st.dropped == 1,
              "reuse: nothing cached - a further non-human push is a no-op (not counted twice)");
        check(!ReplayCaptureAdopt(ACTIVE, HUMAN_A, OTHER, st) && st.cached == 0,
              "reuse: the ghost at the old address is never adopted");
        check(ReplayCaptureAdopt(ACTIVE, HUMAN_A, HUMAN, st) && st.cached == HUMAN_A,
              "reuse: the human re-created at that address is adopted again (mid-run)");
        check(st.changes_while_active == 0, "reuse: re-adoption from empty is not a mid-run change");
    }

    {
        ReplayCaptureState st;
        check(ReplayCaptureAdopt(OFF, HUMAN_A, HUMAN, st) && st.cached == HUMAN_A,
              "idle: the human's recorder is adopted on its first push");
        check(!ReplayCaptureAdopt(OFF, HUMAN_A, HUMAN, st), "same recorder again is not a change");
        check(st.changes_while_active == 0 && st.rejected == 0, "clean counters");
    }
    {
        // transient_pushers_never_hijack: however often a non-human recorder
        // pushes, idle or mid-run, the human stays followed.
        ReplayCaptureState st;
        ReplayCaptureAdopt(OFF, HUMAN_A, HUMAN, st);
        bool any = false;
        for (int i = 0; i < 50; i++) {
            any = ReplayCaptureAdopt(OFF, GHOST, OTHER, st) || any;
            any = ReplayCaptureAdopt(ACTIVE, GARBAGE_OWNER_REC, OTHER, st) || any;
        }
        check(!any && st.cached == HUMAN_A, "transient_pushers_never_hijack: ghost / garbage pushers ignored");
        check(st.rejected == 100, "each ignored pusher is counted (diagnostic)");

        uint32_t last = HUMAN_A;
        LastPusherAdopt(GHOST, last);
        check(last == GHOST, "pre-fix 'last pusher' policy would follow the ghost (documents the bug)");
    }
    {
        // ghost_restart_recreates_recorder_mid_play: REC on HUMAN_A, the PLAY
        // restart rebuilds the player set, HUMAN_B is the new human recorder.
        ReplayCaptureState st;
        ReplayCaptureAdopt(OFF, HUMAN_A, HUMAN, st);
        check(ReplayCaptureAdopt(ACTIVE, HUMAN_B, HUMAN, st) && st.cached == HUMAN_B,
              "ghost_restart_recreates_recorder_mid_play: re-created human recorder adopted mid-run");
        check(st.changes_while_active == 1, "mid-run re-creations are counted (diagnostic)");
        check(!ReplayCaptureAdopt(ACTIVE, HUMAN_B, HUMAN, st), "steady state: no churn while it stays put");

        uint32_t frozen = 0;
        FreezeAdopt(OFF, HUMAN_A, frozen);
        check(!FreezeAdopt(ACTIVE, HUMAN_B, frozen) && frozen == HUMAN_A,
              "pre-fix 'freeze' policy would keep the dead recorder (documents the bug)");
    }
    {
        ReplayCaptureState st;
        check(!ReplayCaptureAdopt(ACTIVE, GHOST, OTHER, st) && st.cached == 0,
              "nothing cached and a non-human pusher: still nothing (never follow a ghost)");
        check(ReplayCaptureAdopt(ACTIVE, HUMAN_A, HUMAN, st) && st.changes_while_active == 0,
              "first human recorder mid-run is an adoption, not a re-creation");
        check(!ReplayCaptureAdopt(ACTIVE, 0, HUMAN, st) && st.cached == HUMAN_A,
              "a null ECX never replaces a live recorder");
    }

    {
        // Owner classification against a fake process image. Layout as in the
        // live game (2026-09-02 afternoon scan, Forest Easy Time Attack):
        // recorder+0x84 -> owner, owner+0x14C -> recorder, [owner] = vtable.
        // The first identity test ([[owner+0x1B8]+0x590] == keyboard object)
        // was a heap-adjacency artifact and matched NOTHING in a fresh process
        // - the DLL then recorded all-zero coordinates and a zero-vs-zero judge
        // "passed". Class identity has no offset to get wrong.
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
        ReplayIdentityEnv env{};
        env.player_vtable = PLAYER_VT;
        env.ghost_vtable = GHOST_VT;
        ReplayIdentityTrace t{};

        check(ClassifyRecorderOwner(HUMAN_REC, env, read, &t) == OWNER_HUMAN,
              "identity: the Player's linked recorder is the human's");
        check(t.owner == HUMAN && t.owner_vtable == PLAYER_VT && t.owner_recorder == HUMAN_REC,
              "identity: trace carries owner / vtable / back-link");
        check(ClassifyRecorderOwner(GHOST_REC, env, read, &t) == OWNER_GHOST,
              "identity: a Ghost_Player's recorder is a ghost");
        check(ClassifyRecorderOwner(AI_REC, env, read, &t) == OWNER_OTHER,
              "identity: an AI_Player's recorder is 'other' (never adopted)");
        check(ClassifyRecorderOwner(DEAD_REC, env, read, &t) == OWNER_UNLINKED,
              "identity: the human's PREVIOUS recorder is unlinked once the player set was rebuilt");
        check(ClassifyRecorderOwner(0x0C460630, env, read, &t) == OWNER_UNREADABLE,
              "identity: a pusher with an unreadable owner is rejected");
        mem[0x0C460630 + 0x84] = 2;  // the garbage-owner pusher seen around restarts
        check(ClassifyRecorderOwner(0x0C460630, env, read, &t) == OWNER_UNREADABLE && t.owner == 2,
              "identity: owner == 2 (garbage) is rejected, traced");
        check(ClassifyRecorderOwner(0, env, read, &t) == OWNER_UNREADABLE,
              "identity: a null ECX is rejected");
        ReplayIdentityEnv none{};
        check(ClassifyRecorderOwner(HUMAN_REC, none, read, &t) == OWNER_OTHER,
              "identity: without a resolved Player vtable nothing is ever human");
        check(ReplayOwnerKindName(OWNER_HUMAN)[0] == 'h' && ReplayOwnerKindName(OWNER_GHOST)[0] == 'g',
              "identity: kind names for the ring log");
    }

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
