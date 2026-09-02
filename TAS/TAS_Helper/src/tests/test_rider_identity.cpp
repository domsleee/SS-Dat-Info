// Unit tests for the rider stamp's pure helpers (rider_identity_parse.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)

#include "../rider_identity_parse.hpp"
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

int main() {
    std::printf("rider_identity tests:\n");
    using namespace riderparse;

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

    check(IsPrintableAscii("Vincent", 7), "printable ASCII accepted");
    check(!IsPrintableAscii("Vin\x01ent", 7), "control byte rejected (dead object garbage)");
    check(!IsPrintableAscii("\xC3\xA9", 2), "non-ASCII rejected");
    check(!IsPrintableAscii("", 0), "empty rejected");

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
