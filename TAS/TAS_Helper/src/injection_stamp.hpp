#pragma once
#include <cstdint>

// The Kernel::Time stamp for an injected key event. Kernel::Time is one 64-bit
// QPC count ({lo, hi}). The observer applies a queued event only once its
// stamp is <= the time the race loop passes to Update, which trails wall time,
// and a later-stamped event holds up every event behind it. A stamp one whole
// hi window (2^32 QPC ticks, about 7 minutes at 10 MHz) before now is always
// in the past, even just after hi rolls over, and stays the same for minutes,
// so a REC and its replay inject the same value. hi == 0 (under one window of
// uptime) gives time zero, also in the past.
inline void InjectionStamp(uint32_t now_hi, uint32_t* lo, uint32_t* hi) {
    *lo = 0;
    *hi = now_hi ? now_hi - 1 : 0;
}
