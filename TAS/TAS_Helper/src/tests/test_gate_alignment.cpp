// Unit tests for the gate-relative replay alignment (gate_alignment.hpp):
// which recorded tick a replay index reads, and the play index a CONT splice
// fires at. Pure logic, no Windows/hook deps.

#include "../gate_alignment.hpp"
#include "check.hpp"

int main() {
    std::printf("gate_alignment tests:\n");

    // --- GateAlignedInputSource: recorded index for a play index -------------
    // Recording: gate at 299, 500 ticks.
    check(GateAlignedInputSource(5, 0, 299, 500) == 5,
          "alignment_preserves_early_input_timing");
    check(GateAlignedInputSource(290, 0, 299, 500) == 290,
          "alignment_preserves_input_before_safety_window");
    check(GateAlignedInputSource(291, 0, 299, 500) == 299,
          "alignment_holds_gate_mask_in_safety_window");
    check(GateAlignedInputSource(297, 297, 299, 500) == 299,
          "alignment_maps_early_live_gate_to_recorded_gate");
    check(GateAlignedInputSource(302, 301, 299, 500) == 300,
          "alignment_maps_positive_gate_offset");
    check(GateAlignedInputSource(502, 301, 299, 500) == GATE_ALIGN_INVALID_SOURCE,
          "alignment_rejects_source_after_recording");
    check(GateAlignedInputSource(31, 0, 40, 500) == 31,
          "short_countdown_preserves_input_before_hold");
    check(GateAlignedInputSource(32, 0, 40, 500) == 40,
          "short_countdown_holds_gate_mask_in_hold");

    // --- GateAlignedSplicePos: play index of a CONT splice --------------------
    // A splice at rec 500 is 201 ticks past the recording's gate (299); the
    // replay reaches that state 201 ticks past ITS gate.
    check(GateAlignedSplicePos(500, 301, 299) == 502, "splice_follows_a_late_live_gate");
    check(GateAlignedSplicePos(500, 297, 299) == 498, "splice_follows_an_early_live_gate");
    check(GateAlignedSplicePos(500, 299, 299) == 500, "splice_unchanged_when_the_gates_coincide");
    check(GateAlignedSplicePos(300, 301, 299) == 302, "one_tick_past_the_gate_is_aligned");
    // Unaligned fallbacks: the plain arm-relative frame.
    check(GateAlignedSplicePos(500, 0, 299) == 500, "no_live_gate_yet_falls_back_to_arm_relative");
    check(GateAlignedSplicePos(500, 301, 0) == 500, "no_recorded_gate_falls_back_to_arm_relative");
    check(GateAlignedSplicePos(299, 301, 299) == 299, "splice_at_the_gate_is_not_aligned");
    check(GateAlignedSplicePos(100, 301, 299) == 100, "splice_inside_the_countdown_is_not_aligned");

    return FinishTests();
}
