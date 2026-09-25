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

    // --- REC/PLAY symmetry block (pause-exempt) ------------------------------
    check(ShouldBlockRealInput(G{REC, false, false, false, false}) == true,
          "rec_blocks_when_running");
    check(ShouldBlockRealInput(G{PLAY, false, false, false, false}) == true,
          "play_blocks_when_running");
    // Paused during REC: pass through so the pause menu is navigable.
    check(ShouldBlockRealInput(G{REC, false, false, /*paused*/ true, false}) == false,
          "rec_paused_passes_for_menu_nav");
    // ESC during REC always passes.
    check(ShouldBlockRealInput(G{REC, false, false, false, /*esc*/ true}) == false,
          "rec_lets_esc_through");

    // --- OFF / idle: nothing to block ----------------------------------------
    check(ShouldBlockRealInput(G{OFF, false, false, false, false}) == false,
          "off_idle_passes");
    check(ShouldBlockRealInput(G{OFF, false, false, true, false}) == false,
          "off_paused_passes");

    return FinishTests();
}
