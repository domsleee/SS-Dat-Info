// Unit tests for held_keys.hpp: the held-key array as an input mask, the
// simulation of an injected take's held keys, and the conversion of its
// prefix at a CONT splice.

#include "../held_keys.hpp"
#include "check.hpp"
#include <cstring>

int main() {
    std::printf("held_keys tests:\n");
    uint8_t held[256] = {};

    held[0x3A] = 1;  // LEFT
    held[0x28] = 1;  // JUMP2 only
    held[0x40] = 1;  // not a TAS key
    check(heldkeys::MaskOf(held) == (0x01 | 0x10), "any_jump_code_sets_the_jump_bit");
    std::memset(held, 0, sizeof held);
    held[0x26] = 1;
    check(heldkeys::MaskOf(held) == 0x20, "the_third_shift_code_counts");

    std::memset(held, 0, sizeof held);
    held[0x40] = 1;
    heldkeys::Write(held, 0x02 | 0x20);
    check(held[0x3B] == 1 && held[0x24] == 1 && held[0x25] == 1 && held[0x26] == 1,
          "write_sets_every_code_of_a_bit");
    check(held[0x3A] == 0 && held[0x27] == 0 && held[0x28] == 0 && held[0x29] == 0,
          "write_clears_the_other_keys");
    check(held[0x40] == 1, "write_leaves_non_tas_keys_alone");
    check(heldkeys::MaskOf(held) == (0x02 | 0x20), "write_then_read_round_trips");

    std::memset(held, 0, sizeof held);
    held[0x28] = 1;
    held[0x25] = 1;
    heldkeys::WriteReal(held, 0x10 | 0x20 | 0x01);
    check(held[0x27] == 1 && held[0x24] == 1 && held[0x3A] == 1, "real_write_sets_the_real_codes");
    check(held[0x28] == 0 && held[0x29] == 0 && held[0x25] == 0 && held[0x26] == 0,
          "real_write_clears_the_aliases");
    held[0x27] = 0;  // the player's Ctrl key-up
    held[0x24] = 0;  // and Shift
    check(heldkeys::MaskOf(held) == 0x01, "real_key_ups_release_what_a_real_write_set");

    std::memset(held, 0, sizeof held);
    heldkeys::Write(held, 0x01 | 0x10 | 0x20);
    heldkeys::Clear(held, 0x10 | 0x20);
    check(heldkeys::MaskOf(held) == 0x01, "clear_releases_only_its_bits");
    check(held[0x27] == 0 && held[0x28] == 0 && held[0x29] == 0 && held[0x24] == 0,
          "clear_releases_every_code_of_a_bit");

    check(heldkeys::IsTasCode(0x3A) && heldkeys::IsTasCode(0x29) && heldkeys::IsTasCode(0x26),
          "arrows_and_every_modifier_alias_are_tas_codes");
    check(!heldkeys::IsTasCode(0x48) && !heldkeys::IsTasCode(0x58) && !heldkeys::IsTasCode(0x2A),
          "esc_f5_alt_are_not");

    // --- Simulating an injected take ------------------------------------------
    {
        // An injected mask is held from the next tick.
        const uint8_t log[] = {0x01, 0x01, 0x00, 0x00};
        uint8_t out[4];
        heldkeys::SimulateInjected(log, 4, out);
        check(out[0] == 0 && out[1] == 0x01 && out[2] == 0x01 && out[3] == 0,
              "injected_mask_is_held_from_the_next_tick");
    }
    {
        // Release LEFT and press RIGHT on one tick: the release applies, the
        // press right behind it waits one more tick.
        const uint8_t log[] = {0x01, 0x01, 0x02, 0x02, 0x02};
        uint8_t out[5];
        heldkeys::SimulateInjected(log, 5, out);
        check(out[0] == 0 && out[1] == 0x01 && out[2] == 0x01 && out[3] == 0 && out[4] == 0x02,
              "two_changes_on_one_tick_land_a_tick_apart");
    }
    {
        // Three changes on one tick: the first and third apply, the second waits.
        const uint8_t log[] = {0x01 | 0x02 | 0x04, 0x07, 0x07};
        uint8_t out[3];
        heldkeys::SimulateInjected(log, 3, out);
        check(out[1] == (0x01 | 0x04) && out[2] == 0x07, "every_other_queued_change_waits");
    }
    {
        // A press and its release on consecutive ticks both apply in order.
        const uint8_t log[] = {0x10, 0x00, 0x00};
        uint8_t out[3];
        heldkeys::SimulateInjected(log, 3, out);
        check(out[0] == 0 && out[1] == 0x10 && out[2] == 0, "a_one_tick_tap_is_held_one_tick");
    }

    // --- Converting a prefix ---------------------------------------------------
    {
        // Indices 0..1 were never replayed, 2..4 were observed.
        uint8_t log[5] = {0x01, 0x01, 0x02, 0x02, 0x02};
        const uint8_t observed[5] = {0, 0, 0x01, 0x00, 0x02};
        const uint8_t seen[5] = {0, 0, 1, 1, 1};
        uint8_t scratch[5];
        const uint32_t off = heldkeys::ConvertInjectedPrefix(log, observed, seen, 5, scratch);
        check(log[0] == 0 && log[1] == 0x01, "unobserved_ticks_take_the_simulation");
        check(log[2] == 0x01 && log[3] == 0x00 && log[4] == 0x02, "observed_ticks_take_the_observation");
        check(off == 0, "a_faithful_replay_agrees_with_the_simulation");
    }
    {
        uint8_t log[2] = {0x01, 0x01};
        const uint8_t observed[2] = {0x08, 0x08};
        const uint8_t seen[2] = {1, 1};
        uint8_t scratch[2];
        check(heldkeys::ConvertInjectedPrefix(log, observed, seen, 2, scratch) == 2,
              "disagreements_are_counted");
        check(log[0] == 0x08, "the_observation_wins");
    }
    uint8_t empty[1] = {0x3F};
    uint8_t scratch[1];
    heldkeys::ConvertInjectedPrefix(empty, empty, empty, 0, scratch);
    check(empty[0] == 0x3F, "zero_length_prefix_is_untouched");

    return FinishTests();
}
