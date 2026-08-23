#pragma once
#include <cstdint>

inline constexpr uint32_t GATE_ALIGN_INVALID_SOURCE = 0xFFFFFFFFu;

// The product controller arms immediately after a settled restart. Deliberate
// 0..40 ms arm-delay sweeps moved the live gate by at most four cycles. Keep a
// much wider window so the gate cycle always receives the recording's gate
// mask, while earlier pre-gate transitions retain their original timing.
inline constexpr uint32_t GATE_ALIGN_PRE_GATE_LEAD = 64u;

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
