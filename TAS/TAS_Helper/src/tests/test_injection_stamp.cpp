// Unit tests for injection_stamp.hpp: an injected key event's stamp must never
// be later than the observer's time, which trails wall time.

#include "../injection_stamp.hpp"
#include "check.hpp"

static uint64_t Join(uint32_t lo, uint32_t hi) { return ((uint64_t)hi << 32) | lo; }

int main() {
    std::printf("injection_stamp tests:\n");
    uint32_t lo = 1, hi = 1;

    InjectionStamp(5, &lo, &hi);
    check(lo == 0 && hi == 4, "stamp_is_one_window_back");

    // Just after hi rolls over, the race loop's time can still be in the old
    // window. The old floored stamp {0, now.hi} was later than it.
    const uint64_t now = Join(0x00000100, 7);
    const uint64_t observer = Join(0xFFFFF000, 6);  // trails wall time by a few ms
    InjectionStamp(7, &lo, &hi);
    check(Join(lo, hi) <= observer, "stamp_not_later_than_a_trailing_observer_across_rollover");
    check(Join(0, 7) > observer, "the_old_floored_stamp_was_later");
    check(Join(lo, hi) < now, "stamp_is_in_the_past");

    InjectionStamp(0, &lo, &hi);
    check(lo == 0 && hi == 0, "first_window_stamps_time_zero");

    uint32_t lo2 = 9, hi2 = 9;
    InjectionStamp(7, &lo2, &hi2);
    check(lo2 == lo && hi2 == 6, "same_window_same_stamp");

    return FinishTests();
}
