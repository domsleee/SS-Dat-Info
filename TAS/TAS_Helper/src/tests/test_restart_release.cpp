// Unit tests for the F5 handler-gate policy (restart_release_policy.hpp).
// Pure logic, no Windows/hook deps - compile + run standalone:
//   just test_dll      (from repo root)
//
// A physical F5 held past the keyboard's repeat delay re-sets the key on
// every autorepeat and the game restarts the level again each time (six or
// seven restarts per long tap, measured 2026-09-09). The gate lets one down
// through per F5_MIN_SPACING_MS and swallows the matching ups, so the game
// never sees an unbalanced up.

#include "../restart_release_policy.hpp"
#include "check.hpp"
#include <cstdio>

int main() {
    std::printf("restart_release tests:\n");
    const uint32_t S = F5_MIN_SPACING_MS;

    check(F5DownAccepted(1000, 0, S), "first ever press is accepted");
    check(F5DownAccepted(1000 + S, 1000, S), "a press exactly one spacing later is accepted");
    check(!F5DownAccepted(1000 + S - 1, 1000, S), "a press just inside the spacing is a repeat");
    check(!F5DownAccepted(1030, 1000, S), "an autorepeat 30 ms later is swallowed");
    check(F5DownAccepted(1000 + 5 * S, 1000, S), "a press much later is accepted");
    // GetTickCount wraps every 49.7 days; the difference must still be right.
    check(F5DownAccepted(100, 0xFFFFFE00u, S), "spacing survives a tick-count wraparound");
    check(!F5DownAccepted(50, 0xFFFFFFF0u, S), "a repeat across the wraparound is still a repeat");

    uint32_t swallowed = 0;
    check(!F5UpSwallowed(swallowed), "an up with no swallowed down passes");
    swallowed = 2;
    check(F5UpSwallowed(swallowed) && swallowed == 1, "an up pairs with one swallowed down");
    check(F5UpSwallowed(swallowed) && swallowed == 0, "...and the next with the other");
    check(!F5UpSwallowed(swallowed), "the up after that passes again");

    return FinishTests();
}
