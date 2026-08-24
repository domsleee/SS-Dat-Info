#pragma once
#include <cstdint>

inline constexpr uint32_t GATE_ALIGN_INVALID_SOURCE = 0xFFFFFFFFu;

// The product controller arms immediately after a settled restart. Deliberate
// 0..40 ms arm-delay sweeps moved the live gate by at most four cycles. The
// hold window exists so the gate cycle always receives the recording's gate
// mask wherever the live gate lands — but every recorded input transition
// INSIDE the window is REPLACED by that mask, so it must stay as narrow as
// the jitter allows: 8 is twice the observed worst case, and a recorded
// press/release pulse 9+ frames before the gate keeps its original timing.
// (Was 64, which silently deleted legitimate transitions from a 640 ms
// window; the four shipped test recordings hold their gate mask stable for
// 78-140 frames, so they behave identically under either value.)
inline constexpr uint32_t GATE_ALIGN_PRE_GATE_LEAD = 8u;

// The play-index at which a CONT splice must fire when the prefix is
// gate-aligned. The recording's splice is at rec-index continue_from_frame,
// which is (continue_from_frame - rec_gate) ticks past the recording's gate;
// the replay reaches the equivalent state that many ticks past ITS gate.
//
// Falls back to the plain arm-relative continue_from_frame when alignment is
// off, the gate has not fired yet, or the splice is at/inside the countdown
// (where the boarder is stationary and there is nothing to align). Unaligned
// CONT is therefore byte-identical to before.
inline uint32_t GateAlignedSplicePos(uint32_t continue_from_frame,
                                     uint32_t live_gate, uint32_t rec_gate) {
    if (rec_gate == 0 || live_gate == 0 || continue_from_frame <= rec_gate) {
        return continue_from_frame;
    }
    return live_gate + (continue_from_frame - rec_gate);
}

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
