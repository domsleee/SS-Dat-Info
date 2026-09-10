#pragma once
#include <atomic>

// The heartbeat only suggests that a cycle is frozen. It is not exclusion:
// a new game cycle can begin while the worker is applying STOP.
class CycleStopGuard {
    std::atomic_flag& flag;
    bool owns;
    bool entered;
public:
    explicit CycleStopGuard(std::atomic_flag& flag, bool acquire = true)
        : flag(flag), owns(acquire && !flag.test_and_set(std::memory_order_acquire)),
          entered(!acquire || owns) {}
    ~CycleStopGuard() { if (owns) flag.clear(std::memory_order_release); }
    explicit operator bool() const { return entered; }
    CycleStopGuard(const CycleStopGuard&) = delete;
    CycleStopGuard& operator=(const CycleStopGuard&) = delete;
};
