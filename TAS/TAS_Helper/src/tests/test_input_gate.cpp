// Unit tests for the cave1c keyboard-gate policy (input_gate.hpp).
// Pure logic, no Windows/hook deps — compile + run standalone:
//   just test_dll      (from repo root)
// or:
//   cl /EHsc /std:c++17 /Fe:test_input_gate.exe test_input_gate.cpp && test_input_gate.exe
//
// The headline case is `cont_block_beats_pause` — the held-key-during-CONT
// regression (f7553f0): a Continue's F5 reload / post-finish state stalls
// Supreme::Cycle (game_paused), and the pause passthrough used to leak live
// input through then, corrupting the spawn. This test FAILS on the pre-fix
// policy (which required !game_paused for every block) and passes on the fix.

#include "../input_gate.hpp"
#include "../gate_alignment.hpp"
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
    using G = InputGateInputs;
    constexpr uint32_t OFF = 0, REC = 1, PLAY = 2;

    std::printf("input_gate tests:\n");

    // --- The regression this file exists for ---------------------------------
    // CONT in flight + cycle stalled (F5 reload / post-finish): live input MUST
    // be blocked. Pre-fix this returned false (block required !game_paused) and
    // a held ctrl/Enter reached the spawn → F5 spawned at the finish line.
    check(ShouldBlockRealInput(G{OFF, /*cont*/ true, /*inj*/ false,
                                 /*paused*/ true, /*esc*/ false}) == true,
          "cont_block_beats_pause (the held-key-during-CONT regression)");

    // CONT in flight, cycle ticking normally: also blocked.
    check(ShouldBlockRealInput(G{OFF, true, false, false, false}) == true,
          "cont_block_when_running");

    // CONT in flight but it's our OWN injection: must pass (cave2 owns input).
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

    // --- Gate-relative replay source mapping --------------------------------
    check(GateAlignedInputSource(5, 0, 299, 500) == 5,
          "alignment_preserves_early_input_timing");
    check(GateAlignedInputSource(234, 0, 299, 500) == 234,
          "alignment_preserves_input_before_safety_window");
    check(GateAlignedInputSource(235, 0, 299, 500) == 299,
          "alignment_holds_gate_mask_in_safety_window");
    check(GateAlignedInputSource(297, 297, 299, 500) == 299,
          "alignment_maps_early_live_gate_to_recorded_gate");
    check(GateAlignedInputSource(302, 301, 299, 500) == 300,
          "alignment_maps_positive_gate_offset");
    check(GateAlignedInputSource(502, 301, 299, 500) == GATE_ALIGN_INVALID_SOURCE,
          "alignment_rejects_source_after_recording");
    check(GateAlignedInputSource(0, 0, 40, 500) == 40,
          "short_countdown_holds_gate_mask_from_arm");

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
