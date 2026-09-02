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
// The identity check (owner's controller holds the keyboard object) resolves
// both; the pre-fix policies are kept here only so the regressions are
// demonstrable.

#include "../replay_capture_policy.hpp"
#include <cstdio>

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

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
