#pragma once
#include <cstdint>

inline constexpr uint32_t GATE_ALIGN_INVALID_SOURCE = 0xFFFFFFFFu;

// Gate-relative replay; see DESIGN.md "Replaying from the gate".

// An aligned splice past the gate while the live gate has not fired yet. No
// play index reaches it.
inline constexpr uint32_t GATE_ALIGN_SPLICE_PENDING = 0xFFFFFFFFu;

// Ticks before the recording's gate from which a replay holds the gate mask
// until its own gate fires. The live gate lands within about four ticks; the
// window replaces recorded transitions, so it is kept narrow (twice that).
inline constexpr uint32_t GATE_ALIGN_PRE_GATE_LEAD = 8u;

// The play index at which a CONT splice fires: as many ticks past the live
// gate as continue_from_frame is past the recording's gate. Unaligned, or a
// splice inside the countdown, uses continue_from_frame as is. PENDING while
// the live gate hasn't fired, so the replay can't park before it.
inline uint32_t GateAlignedSplicePos(uint32_t continue_from_frame,
                                     uint32_t live_gate, uint32_t rec_gate) {
    if (rec_gate == 0 || continue_from_frame <= rec_gate) {
        return continue_from_frame;
    }
    if (live_gate == 0) {
        return GATE_ALIGN_SPLICE_PENDING;
    }
    return live_gate + (continue_from_frame - rec_gate);
}

// Ticks the tick cave may run this frame toward a CONT splice. Once approved
// at the splice, allow one tick: it belongs to REC, and a catch-up batch would
// spill into the recording. While pending, stop at the pre-gate window, then
// step one tick per frame so a batch can't overshoot a splice just past the
// gate.
inline uint32_t ContinueSpliceTickLimit(uint32_t pos, uint32_t splice, bool approved,
                                        uint32_t rec_gate) {
    if (splice == GATE_ALIGN_SPLICE_PENDING) {
        const uint32_t window =
            rec_gate > GATE_ALIGN_PRE_GATE_LEAD ? rec_gate - GATE_ALIGN_PRE_GATE_LEAD : 0u;
        return pos < window ? window - pos : 1u;
    }
    return pos < splice ? splice - pos : (approved ? 1u : 0u);
}

// The input_log index to replay at play index pos. Unaligned: pos. Before the
// live gate: pos, then the gate mask from the pre-gate lead on. After it: the
// same distance past the recording's gate.
inline uint32_t GateAlignedInputSource(uint32_t pos, uint32_t live_gate,
                                       uint32_t rec_gate, uint32_t recorded_count) {
    if (rec_gate == 0 || rec_gate >= recorded_count) {
        return pos < recorded_count ? pos : GATE_ALIGN_INVALID_SOURCE;
    }

    if (live_gate == 0) {
        const uint32_t hold_from =
            rec_gate > GATE_ALIGN_PRE_GATE_LEAD ? rec_gate - GATE_ALIGN_PRE_GATE_LEAD : 0u;
        const uint32_t src = pos < hold_from ? pos : rec_gate;
        return src < recorded_count ? src : GATE_ALIGN_INVALID_SOURCE;
    }

    const int64_t offset = static_cast<int64_t>(pos) - static_cast<int64_t>(live_gate);
    const int64_t src = static_cast<int64_t>(rec_gate) + offset;
    return src < 0 || src >= static_cast<int64_t>(recorded_count)
               ? GATE_ALIGN_INVALID_SOURCE
               : static_cast<uint32_t>(src);
}
