// Unit tests for the race timer's HUD-line table (race_timer_table.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)
//
// Regression from 2026-09-02 (Forest Easy, TAS_RACE_DIAG=1):
//   stale_line_from_the_previous_level_does_not_make_the_next_race_ambiguous
//     A menu trip rebuilds the HUD. The old player line stayed in the table
//     with adv=7473, the new race's line advanced too, Classify saw two
//     advancing lines and published nothing ("ambiguous") for every race
//     after the first. By the fourth trip all eight slots held dead objects.
//     game_in_game never drops at the menu, so the epoch reset never fired.

#include "../race_timer_table.hpp"
#include <cstdio>

using namespace racetimer;

static int g_failures = 0;

static void check(bool cond, const char* name) {
    if (cond) {
        std::printf("  ok   %s\n", name);
    } else {
        std::printf("  FAIL %s\n", name);
        g_failures++;
    }
}

// A live race on `line`: the HUD is appended once per clock tick, the timer
// advances one centisecond per tick from `startClk`. Returns the last verdict.
static Verdict RunLine(Table& t, uint32_t line, uint32_t& clk, uint32_t& tick, int samples,
                       int startCs = 0) {
    Verdict v{MAXU, MAXU, ""};
    for (int i = 0; i < samples; i++) {
        clk++;
        tick++;
        v = t.Sample(line, startCs + i, clk, tick);
    }
    return v;
}

int main() {
    std::printf("race_timer tests:\n");
    constexpr uint32_t PLAYER_A = 0x0B181928, RECORD_A = 0x0B180E38;
    constexpr uint32_t PLAYER_B = 0x0DADA348, RECORD_B = 0x0DADA7F8;

    {
        Table t;
        uint32_t clk = 790, tick = STALE_TICKS + 1;
        // A constant record line ("00:19:79") interleaved with the player.
        Verdict v{MAXU, MAXU, ""};
        for (int i = 0; i < LOCK_FRAMES + 2; i++) {
            clk++;
            tick++;
            t.Sample(RECORD_A, 1979, clk, tick);
            v = t.Sample(PLAYER_A, i, clk, tick);
        }
        check(v.cs == (uint32_t)(LOCK_FRAMES + 1), "the advancing line locks as the player after LOCK_FRAMES samples");
        check(t.playerLine == PLAYER_A, "latched on the player line");
        check(v.start == 790 - 0 + 1 && v.start == 791, "start_ts = clock - cs once (clock-cs) settled");
        int used = t.Used();
        check(used == 2, "record line + player line occupy two slots");
        (void)used;
    }
    {
        // stale_line_from_the_previous_level_does_not_make_the_next_race_ambiguous
        Table t;
        uint32_t clk = 790, tick = STALE_TICKS + 1;
        RunLine(t, PLAYER_A, clk, tick, 100);
        check(t.playerLine == PLAYER_A && t.adv[t.Slot(PLAYER_A, tick)] >= LOCK_FRAMES,
              "first race: player A locked with a long advancing history");
        // Menu trip: nothing is appended for a while, then a NEW level's HUD
        // lines appear (fresh objects) and the new player line starts advancing.
        tick += STALE_TICKS + 50;
        clk += STALE_TICKS + 50;
        Verdict v{MAXU, MAXU, ""};
        for (int i = 0; i < LOCK_FRAMES + 2; i++) {
            clk++;
            tick++;
            t.Sample(RECORD_B, 1979, clk, tick);
            v = t.Sample(PLAYER_B, i, clk, tick);
        }
        bool aStillThere = false;
        for (int i = 0; i < SLOTS; i++) aStillThere |= t.line[i] == PLAYER_A;
        check(!aStillThere, "the previous level's player line was evicted as stale");
        check(t.playerLine == PLAYER_B, "the new race's line is the player");
        check(v.cs == (uint32_t)(LOCK_FRAMES + 1) && v.reason[0] == 'l',
              "second race publishes its own time (was: ambiguous forever)");
    }
    {
        // Two lines advancing AT THE SAME TIME (ghost / second racer) are still
        // ambiguous - staleness only removes lines that stopped being sampled.
        Table t;
        uint32_t clk = 100, tick = STALE_TICKS + 1;
        Verdict v{MAXU, MAXU, ""};
        for (int i = 0; i < LOCK_FRAMES + 2; i++) {
            clk++;
            tick++;
            t.Sample(PLAYER_A, i, clk, tick);
            v = t.Sample(PLAYER_B, i, clk, tick);
        }
        // No lock is ever acquired ("acquiring"); a lock that existed before
        // the second line appeared is dropped ("ambiguous"). Either way:
        // nothing is published and nothing is latched.
        check(v.cs == MAXU && t.playerLine == 0,
              "two simultaneously advancing lines publish nothing (no latch)");
        // A line's advancing history only ends with a backward jump or
        // eviction (a frozen line keeps its history so the player's own
        // finish freeze stays latched). B goes silent while A keeps being
        // sampled: after STALE_TICKS B is evicted and A locks.
        v = RunLine(t, PLAYER_A, clk, tick, STALE_TICKS + 2, LOCK_FRAMES + 2);
        check(t.playerLine == PLAYER_A && v.cs == (uint32_t)(LOCK_FRAMES + 2 + STALE_TICKS + 1),
              "the survivor locks once the other line is gone");
        // B comes back and advances again: the existing lock is dropped.
        v = RunLine(t, PLAYER_B, clk, tick, LOCK_FRAMES + 1, 500);
        check(v.cs == MAXU && v.reason[0] == 'a' && v.reason[1] == 'm' && t.playerLine == 0,
              "a second advancing line drops an existing lock as ambiguous");
    }
    {
        // Slot exhaustion: eight dead lines, a ninth still gets a slot.
        Table t;
        uint32_t tick = STALE_TICKS + 1;
        for (uint32_t l = 1; l <= (uint32_t)SLOTS; l++) t.Sample(0x1000 * l, 1979, 10, tick);
        check(t.Used() == SLOTS, "table full");
        check(t.Slot(0x9999, tick) == -1, "a fresh table full of live lines refuses a ninth");
        tick += STALE_TICKS + 1;
        int i = t.Slot(0x9999, tick);
        check(i >= 0 && t.line[i] == 0x9999, "once the eight went stale the ninth recycles a slot");
        check(t.Used() == 1 || t.Used() == SLOTS,
              "Slot() recycles exactly one slot (Evict() clears the rest on the next sample)");
        t.Sample(0x9999, 5, 11, tick);
        check(t.Used() == 1, "the next sample evicts every other stale line");
    }
    {
        // Latch survives the finish freeze and the (clock-cs) wobble.
        Table t;
        uint32_t clk = 500, tick = STALE_TICKS + 1;
        RunLine(t, PLAYER_A, clk, tick, 50);
        Verdict v{MAXU, MAXU, ""};
        for (int i = 0; i < 20; i++) {      // frozen at 49 after the line
            clk++;
            tick++;
            v = t.Sample(PLAYER_A, 49, clk, tick);
        }
        check(v.cs == 49 && t.playerLine == PLAYER_A, "frozen finish time stays published (latched)");
        clk++; tick++;
        v = t.Sample(PLAYER_A, 49, clk + 1, tick);   // +1 wobble in (clock-cs)
        check(v.cs == 49, "a +/-1 (clock-cs) wobble does not blank");
        // A restart (cs jumps back) resets the advancing history; the line
        // re-locks once it advances again.
        clk++; tick++;
        v = t.Sample(PLAYER_A, 0, clk, tick);
        check(t.adv[t.Slot(PLAYER_A, tick)] == 0, "a backward jump (F5) resets the advance counter");
        v = RunLine(t, PLAYER_A, clk, tick, LOCK_FRAMES + 1, 1);
        check(v.cs == (uint32_t)(LOCK_FRAMES + 1), "re-locks after the restart");
    }
    {
        // Reset drops everything.
        Table t;
        uint32_t clk = 1, tick = STALE_TICKS + 1;
        RunLine(t, PLAYER_A, clk, tick, 20);
        t.Reset();
        check(t.Used() == 0 && t.playerLine == 0 && t.playerStartTs == MAXU, "Reset clears slots and the latch");
        check(t.Classify().cs == MAXU, "nothing to publish after a reset");
    }

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
