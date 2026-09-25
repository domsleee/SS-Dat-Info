#pragma once
#include <cstdint>

// The race timer's HUD-line table (pure logic, tested in
// tests/test_race_timer.cpp). Time strings are keyed by their text-line
// object; the player's line is the one whose value advances (a par line is
// constant). Every race rebuilds the HUD, so lines not sampled for
// STALE_TICKS are evicted, or old lines would make the next pick ambiguous.
namespace racetimer {

constexpr int      SLOTS = 8;
constexpr int      LOCK_FRAMES = 8;     // advancing-cs samples to LOCK a line as the player
constexpr int      SETTLE_FRAMES = 3;   // distinct ticks (clock-cs) constant before start_ts is trusted
constexpr uint32_t STALE_TICKS = 200;   // 2 s without a sample: the HUD line no longer exists
constexpr uint32_t MAXU = 0xFFFFFFFFu;

struct Verdict {
    uint32_t cs;        // race time to publish (MAXU = blank)
    uint32_t start;     // start_ts to publish (MAXU = unknown)
    const char* reason; // for the diagnostics log
};

// Single writer: the game thread (Append_Text hook, clock-tick hook).
struct Table {
    uint32_t line[SLOTS] = {};     // text-line object ptrs (0 = free)
    int      cs[SLOTS] = {};       // latest parsed centiseconds
    int      adv[SLOTS] = {};      // consecutive samples where cs ADVANCED (player signal)
    int      start[SLOTS] = {};    // last (clock - cs) & 0xFFFF (= start_ts candidate)
    int      stable[SLOTS] = {};   // consecutive distinct-tick (clock-cs)-constant samples
    uint32_t lastClk[SLOTS] = {};  // clock at last sample (sentinel = MAXU)
    uint32_t seen[SLOTS] = {};     // monotonic tick at last sample (staleness)
    uint32_t playerLine = 0;       // locked player-line object once identified
    uint32_t playerStartTs = MAXU; // cached good start_ts (survives wobble/finish)

    void Reset() {
        for (int i = 0; i < SLOTS; i++) Free(i);
        playerLine = 0;
        playerStartTs = MAXU;
    }

    void Free(int i) {
        line[i] = 0;
        cs[i] = 0;
        adv[i] = 0;
        start[i] = -1;
        stable[i] = 0;
        lastClk[i] = MAXU;
        seen[i] = 0;
    }

    int Used() const {
        int n = 0;
        for (int i = 0; i < SLOTS; i++) n += line[i] != 0;
        return n;
    }

    // Drops lines not sampled within STALE_TICKS, unlatching the player line
    // if it went. Returns the number freed.
    int Evict(uint32_t tick) {
        int freed = 0;
        for (int i = 0; i < SLOTS; i++) {
            if (line[i] == 0) continue;
            if (tick - seen[i] > STALE_TICKS) {
                if (line[i] == playerLine) {
                    playerLine = 0;
                    playerStartTs = MAXU;
                }
                Free(i);
                freed++;
            }
        }
        return freed;
    }

    // Slot holding `l`, or a free or stale one; -1 if none.
    int Slot(uint32_t l, uint32_t tick) {
        for (int i = 0; i < SLOTS; i++)
            if (line[i] == l) return i;
        int pick = -1;
        uint32_t oldest = 0;
        for (int i = 0; i < SLOTS; i++) {
            if (line[i] == 0) { pick = i; break; }
            uint32_t age = tick - seen[i];
            if (age > STALE_TICKS && age >= oldest) { oldest = age; pick = i; }
        }
        if (pick < 0) return -1;
        if (line[pick] == playerLine) {
            playerLine = 0;
            playerStartTs = MAXU;
        }
        Free(pick);
        line[pick] = l;
        seen[pick] = tick;
        return pick;
    }

    // One fresh HUD sample: `l` showed `csNow` at 16-bit clock `clk`, on
    // monotonic tick `tick`. Returns what to publish.
    Verdict Sample(uint32_t l, int csNow, uint32_t clk, uint32_t tick) {
        Evict(tick);
        int i = Slot(l, tick);
        if (i < 0) return Classify();
        seen[i] = tick;

        int prevCs = cs[i];
        bool first = (lastClk[i] == MAXU);
        // A backward jump (F5 / new race) resets the counter; an equal cs
        // (par, or a frozen finish) leaves it unchanged.
        if (!first) {
            int d = csNow - prevCs;
            if (d > 0 && d < 30000) { if (adv[i] < 1000000) adv[i]++; }
            else if (d < 0)         { adv[i] = 0; }
        }
        // start_ts trust: is (clock - cs) constant across distinct clock ticks?
        bool clkMoved = (lastClk[i] != clk);
        int sg = (int)((clk - (uint32_t)csNow) & 0xFFFF);
        if (clkMoved) {
            if (sg == start[i]) {
                if (stable[i] < 1000000) stable[i]++;
            } else {
                start[i] = sg;
                stable[i] = 0;
            }
            lastClk[i] = clk;
        }
        cs[i] = csNow;
        return Classify();
    }

    // Once locked, the player line stays latched through the +/-1 (clock-cs)
    // wobble and the finish freeze; a second advancing line blanks it as
    // ambiguous. start_ts is published only once (clock-cs) settles, so it
    // stays unknown in a replay.
    Verdict Classify() {
        int advCount = 0, best = -1, bestAdv = 0;
        for (int i = 0; i < SLOTS; i++) {
            if (line[i] == 0) continue;
            if (adv[i] >= LOCK_FRAMES) {
                advCount++;
                if (adv[i] > bestAdv) { bestAdv = adv[i]; best = i; }
            }
        }
        if (advCount == 1 && best >= 0) playerLine = line[best];

        int pi = -1;
        if (playerLine != 0)
            for (int i = 0; i < SLOTS; i++)
                if (line[i] == playerLine) { pi = i; break; }
        if (pi < 0) return {MAXU, MAXU, "acquiring"};

        if (advCount >= 2) {
            playerLine = 0;
            playerStartTs = MAXU;
            return {MAXU, MAXU, "ambiguous"};
        }
        if (stable[pi] >= SETTLE_FRAMES)
            playerStartTs = (uint32_t)(start[pi] & 0xFFFF);
        return {(uint32_t)cs[pi], playerStartTs, "latched"};
    }
};

} // namespace racetimer
