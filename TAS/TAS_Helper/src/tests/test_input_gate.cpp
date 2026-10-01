// Unit tests for the key-handler cave's keyboard-gate policy (input_gate.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)
// or:
//   cl /EHsc /std:c++17 /Fe:test_input_gate.exe test_input_gate.cpp && test_input_gate.exe
//
// The key case is `cont_block_beats_pause`: a Continue's F5 reload and
// post-finish state stall Supreme::Cycle (game_paused), and live input must
// stay blocked through them or it corrupts the spawn.

#include "../input_gate.hpp"
#include "check.hpp"
#include <cstdio>

int main() {
    using G = InputGateInputs;
    constexpr uint32_t OFF = 0, REC = 1, PLAY = 2;

    std::printf("input_gate tests:\n");

    // --- CONT block beats the pause passthrough -------------------------------
    // CONT in flight + cycle stalled (F5 reload / post-finish): live input MUST
    // be blocked, or a held key reaches the spawn.
    check(ShouldBlockRealInput(G{OFF, /*cont*/ true, /*inj*/ false,
                                 /*paused*/ true, /*esc*/ false}) == true,
          "cont_block_beats_pause (the held-key-during-CONT regression)");

    // CONT in flight, cycle ticking normally: also blocked.
    check(ShouldBlockRealInput(G{OFF, true, false, false, false}) == true,
          "cont_block_when_running");

    // CONT in flight but it's our OWN injection: must pass (the cycle cave owns input).
    check(ShouldBlockRealInput(G{OFF, true, /*inj*/ true, false, false}) == false,
          "cont_lets_injection_through");

    // CONT in flight but it's ESC: must pass (abort hatch / pause-menu nav).
    check(ShouldBlockRealInput(G{OFF, true, false, true, /*esc*/ true}) == false,
          "cont_lets_esc_through_even_paused");

    // --- PLAY blocks (pause-exempt); REC records the six keys ---------------
    check(ShouldBlockRealInput(G{REC, false, false, false, false, /*tas key*/ true}) == false,
          "rec_passes_a_recorded_key");
    check(ShouldBlockRealInput(G{REC, false, false, false, false, /*tas key*/ false}) == true,
          "rec_blocks_a_key_the_take_cannot_hold");
    check(ShouldBlockRealInput(G{REC, false, false, /*paused*/ true, false, false}) == false,
          "rec_paused_passes_menu_keys");
    check(ShouldBlockRealInput(G{REC, false, false, false, /*esc*/ true, false}) == false,
          "rec_lets_esc_through");
    check(ShouldBlockRealInput(G{REC, false, false, false, false, false, /*release*/ true}) == false,
          "rec_passes_the_release_of_a_key_let_through_in_a_menu");
    check(ShouldBlockRealInput(G{PLAY, false, false, false, false, false, /*release*/ true}) == false,
          "play_passes_the_release_of_a_non_tas_key");
    check(ShouldBlockRealInput(G{PLAY, false, false, false, false, true, /*release*/ true}) == true,
          "play_still_blocks_a_tas_key_release");
    check(IsTasVirtualKey(0x25) && IsTasVirtualKey(0x28) && IsTasVirtualKey(0x10) &&
              IsTasVirtualKey(0x11),
          "arrows_ctrl_shift_are_recorded_keys");
    check(!IsTasVirtualKey(0x74) && !IsTasVirtualKey(0x20) && !IsTasVirtualKey(0x1B),
          "f5_space_escape_are_not");
    check(ShouldBlockRealInput(G{PLAY, false, false, false, false}) == true,
          "play_blocks_when_running");
    // Paused during PLAY: pass through so the pause menu is navigable.
    check(ShouldBlockRealInput(G{PLAY, false, false, /*paused*/ true, false}) == false,
          "play_paused_passes_for_menu_nav");
    // ESC during PLAY always passes.
    check(ShouldBlockRealInput(G{PLAY, false, false, false, /*esc*/ true}) == false,
          "play_lets_esc_through");
    // A CONT's restart still blocks, even for a REC.
    check(ShouldBlockRealInput(G{REC, true, false, false, false, /*tas key*/ true}) == true,
          "cont_blocks_even_a_recorded_key");

    // --- OFF / idle: nothing to block ----------------------------------------
    check(ShouldBlockRealInput(G{OFF, false, false, false, false}) == false,
          "off_idle_passes");
    check(ShouldBlockRealInput(G{OFF, false, false, true, false}) == false,
          "off_paused_passes");

    // --- What counts as a pause (IsMenuPause) --------------------------------
    // The race stalled AND a menu is executing: the pause menu, a dialog.
    check(IsMenuPause(400, true, 16) == true, "pause_menu_over_stalled_race");
    // A long frame with no menu is a hitch, not a pause: keep blocking.
    check(IsMenuPause(400, true, 5000) == false, "hitch_without_menu_is_not_a_pause");
    check(IsMenuPause(400, false, 0) == false, "hitch_before_any_menu_is_not_a_pause");
    // A menu while the race still ticks (in-race HUD painting) is no pause.
    check(IsMenuPause(10, true, 5) == false, "menu_over_running_race_is_not_a_pause");
    check(IsMenuPause(INPUT_GATE_PAUSE_MS, true, 0) == false, "boundary_cycle_age_is_not_stalled");
    check(IsMenuPause(INPUT_GATE_PAUSE_MS + 1, true, INPUT_GATE_PAUSE_MS) == true, "boundary_menu_age_is_fresh");

    return FinishTests();
}
