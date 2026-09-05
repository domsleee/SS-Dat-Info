// Standalone unit tests for the level-path parsing (no game, no Windows APIs).
//
// These cover the decisions that are actually easy to get wrong and that a live
// run exercises only by luck: which segment is the area, whether a garbage
// buffer is rejected, and — most importantly — that the DIFFICULTY segment is
// never trusted, because some tracks share the easy/ shadow asset.
#include "../level_path_parse.hpp"
#include <cstdio>

static int g_fail = 0;

static void check(bool cond, const char* name) {
    std::printf("  %-4s %s\n", cond ? "ok" : "FAIL", name);
    if (!cond) g_fail++;
}

int main() {
    using namespace levelpath;

    std::printf("level_path tests:\n");

    // --- Plausibility: the pointer can land on freed-but-committed heap, which
    // does not fault, so "read without crashing" proves nothing. ---
    check(IsPlausible("data/levels/Forest/Tracks/Easy/Cloudy/shadow.qua"),
          "real level path is plausible");
    check(!IsPlausible(""), "empty string rejected");
    check(!IsPlausible(nullptr), "null rejected");
    check(!IsPlausible("Data/Levels/"), "prefix literal alone rejected (no tracks)");
    check(!IsPlausible("Saved_Data/Available_Levels.txt"),
          "unrelated levels-ish string rejected (no tracks)");
    check(!IsPlausible("\x01\x02garbage\xff"), "binary garbage rejected");

    // --- Segment boundaries, not substrings. ---
    // "Available_Levels" and "Soundtracks" are real names from this game's data
    // directory, and a substring search accepts the pair as a level path. This
    // is the case a live run would only hit by luck, and the reason IsPlausible
    // and AreaFrom both anchor on separators.
    check(!IsPlausible("Available_Levels/Forest/Soundtracks/x"),
          "near-miss rejected: levels/tracks only as SUBSTRINGS");
    check(AreaFrom("Available_Levels/Forest/Soundtracks/x") == -1,
          "...and it yields no area either");
    check(!IsPlausible("mylevels/Forest/Tracks/Easy/x"),
          "levels must start a segment");
    check(!IsPlausible("data/levelsx/Forest/Tracks/Easy/x"),
          "levels must end a segment");
    check(IsPlausible("levels/Forest/tracks/Easy/x"),
          "a segment at the very start of the string still counts");

    // --- Area extraction, the half the path answers reliably. ---
    check(AreaFrom("data/levels/Forest/Tracks/Easy/Cloudy/shadow.qua") == 0, "area Forest");
    check(AreaFrom("data/levels/Alpine/Tracks/Hard/x") == 1, "area Alpine");
    check(AreaFrom("data/levels/village/Tracks/easy/x") == 2, "area Village (lowercase)");
    check(AreaFrom("DATA/LEVELS/VILLAGE/TRACKS/HARD/x") == 2, "area is case-insensitive");

    // Backslash separators (the engine mixes them).
    {
        char p[] = "data\\levels\\Alpine\\Tracks\\Medium\\x";
        check(AreaFrom(p) == 1, "backslash separators");
    }

    // --- Non-Track modes and junk must NOT produce an area. ---
    check(AreaFrom("data/levels/Practice/x") == 3, "Practice is area 3");
    check(AreaFrom("data/levels/Practice/Tracks/Easy/Cloudy/shadow.qua") == 3,
          "practice full path resolves");
    check(AreaFrom("data/levels/Special/x") == -1, "Special is not an area");
    check(AreaFrom("Data/Levels/") == -1, "bare prefix has no area");
    check(AreaFrom("no/paths/here") == -1, "unrelated path has no area");
    check(AreaFrom(nullptr) == -1, "null has no area");

    // A "levels" that is not followed by an area must not stop the search — the
    // real segment can appear later in the string.
    check(AreaFrom("Available_Levels.txt;data/levels/Forest/Tracks/Easy/x") == 0,
          "keeps scanning past a non-area levels match");

    // --- THE CRITICAL PROPERTY. ---
    // Village Hard resolves to the SHARED easy/ shadow asset, so the difficulty
    // segment of this path is a lie. The parser must expose AREA ONLY; anything
    // that read difficulty from here would report Village EASY while the player
    // is on Village HARD. This is why the setup config owns difficulty.
    check(AreaFrom("data/levels/village/Tracks/easy/Cloudy/shadow.qua") == 2,
          "VH path yields area Village (difficulty deliberately not exposed)");

    // --- MatchOne edge cases (used for the area segment). ---
    check(MatchOne("forest", 6, AREAS, 3) == 0, "MatchOne exact");
    check(MatchOne("Forest", 6, AREAS, 3) == 0, "MatchOne folds case");
    check(MatchOne("forestx", 7, AREAS, 3) == -1, "MatchOne rejects longer");
    check(MatchOne("fore", 4, AREAS, 3) == -1, "MatchOne rejects prefix");
    check(MatchOne("", 0, AREAS, 3) == -1, "MatchOne rejects empty");

    // LevelIdFrom: path area (reliable) + setup object difficulty. Practice is
    // the regression that prompted these (2026-09-03): it skips the menu screen
    // that writes the setup object, so the object holds a STALE Arcade
    // selection; the id must still come out 9 from the path alone.
    check(LevelIdFrom(3, "Village", "Hard") == 9, "Practice resolves to 9 despite a stale Village/Hard setup");
    check(LevelIdFrom(3, "", "") == 9, "Practice resolves to 9 with an unreadable setup");
    check(LevelIdFrom(0, "Forest", "Easy") == 0, "Forest Easy");
    check(LevelIdFrom(0, "Forest", "Medium") == 1, "Forest Medium (path can't give difficulty; setup does)");
    check(LevelIdFrom(0, "Forest", "Hard") == 2, "Forest Hard");
    check(LevelIdFrom(2, "Village", "Easy") == 6, "Village Easy");
    check(LevelIdFrom(2, "Village", "Hard") == 8, "Village Hard (path says easy; setup says Hard)");
    check(LevelIdFrom(2, "village", "hard") == 8, "case-folded setup strings");
    check(LevelIdFrom(2, "Forest", "Easy") == -1, "stale setup: its area disagrees with the path area");
    check(LevelIdFrom(0, "", "") == -1, "non-practice with an unreadable setup is unresolved");
    check(LevelIdFrom(0, "Forest", "Xyz") == -1, "unknown difficulty string is unresolved");
    check(LevelIdFrom(-1, "Forest", "Easy") == -1, "no path area => unresolved");
    check(LevelIdFrom(4, "Forest", "Easy") == -1, "out-of-range path area => unresolved");

    if (g_fail == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_fail);
    return 1;
}
