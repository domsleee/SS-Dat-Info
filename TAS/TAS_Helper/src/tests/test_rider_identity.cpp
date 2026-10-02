// Unit tests for the rider stamp's pure helpers (rider_identity_parse.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)

#include "../rider_identity_parse.hpp"
#include "check.hpp"
#include <cstdio>

int main() {
    std::printf("rider_identity tests:\n");
    using namespace riderparse;

    // StringHeaderUsable rejects a wrapped-around length from a dead string.
    check(StringHeaderUsable(0x02000000, 5, 31, 32), "a normal 5-char string fits a 32-byte buffer");
    check(StringHeaderUsable(0x02000000, 31, 31, 32), "len 31 into cap 32 is the largest that fits");
    check(!StringHeaderUsable(0x02000000, 32, 32, 32), "len 32 into cap 32 leaves no room for the NUL");
    check(!StringHeaderUsable(0x02000000, 0xFFFFFFFFu, 0xFFFFFFFFu, 32), "len 0xFFFFFFFF (dead object) is rejected, no wrap");
    check(!StringHeaderUsable(0x02000000, 0xFFFFFFFEu, 0xFFFFFFFFu, 32), "len 0xFFFFFFFE is rejected too");
    check(!StringHeaderUsable(0x02000000, 0, 31, 32), "empty string is not a name");
    check(!StringHeaderUsable(0x02000000, 5, 3, 32), "capacity smaller than size = torn header");
    check(!StringHeaderUsable(0x0000FFFF, 5, 31, 32), "pointer below 64K is not a heap address");
    check(!StringHeaderUsable(0x02000000, 5, 31, 0), "zero-size destination");

    check(CharacterFromName("Vincent") == CHARACTER_VINCENT, "display name (Player_Config) maps to the id");
    check(CharacterFromName("vincent") == CHARACTER_VINCENT, "data folder name (loadout) maps to the same id");
    check(CharacterFromName("Keith") == CHARACTER_KEITH && CharacterFromName("KEITH") == CHARACTER_KEITH,
          "case-insensitive");
    check(CharacterFromName("Akiko") == CHARACTER_AKIKO && CharacterFromName("Karl") == CHARACTER_KARL &&
              CharacterFromName("Mike") == CHARACTER_MIKE && CharacterFromName("Ulrika") == CHARACTER_ULRIKA,
          "every selectable rider has an id");
    check(CharacterFromName("Bulk_7") == CHARACTER_OTHER && CharacterFromName("guide") == CHARACTER_OTHER,
          "a name outside the table is OTHER, never UNKNOWN");
    check(CharacterFromName("") == CHARACTER_UNKNOWN && CharacterFromName(nullptr) == CHARACTER_UNKNOWN,
          "no name = not resolved");
    check(CharacterFromName("Vince") == CHARACTER_OTHER && CharacterFromName("Vincent2") == CHARACTER_OTHER,
          "prefix / suffix are not matches");

    check(StanceForRider(0, "Keith", "Keith") == 0, "regular stance accepted for the live rider");
    check(StanceForRider(1, "keith", "KEITH") == 1, "goofy stance accepted case-insensitively");
    check(StanceForRider(1, "Keith", "Vincent") == STANCE_UNKNOWN,
          "stance rejected when setup and live rider differ");
    check(StanceForRider(2, "Keith", "Keith") == STANCE_UNKNOWN,
          "out-of-range stance is unknown, never assumed regular");
    check(StanceForRider(0, nullptr, "Keith") == STANCE_UNKNOWN &&
              StanceForRider(0, "Keith", nullptr) == STANCE_UNKNOWN,
          "stance needs both character names");

    check(IsPrintableAscii("Vincent", 7), "printable ASCII accepted");
    check(!IsPrintableAscii("Vin\x01ent", 7), "control byte rejected (dead object garbage)");
    check(!IsPrintableAscii("\xC3\xA9", 2), "non-ASCII rejected");
    check(!IsPrintableAscii("", 0), "empty rejected");

    return FinishTests();
}
