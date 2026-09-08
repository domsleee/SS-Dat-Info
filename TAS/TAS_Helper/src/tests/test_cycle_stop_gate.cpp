#include "../cycle_stop_gate.hpp"
#include "check.hpp"
#include <thread>

int main() {
    std::atomic_flag flag = ATOMIC_FLAG_INIT;
    {
        CycleStopGuard worker(flag);
        check(bool(worker), "worker claims cleanup before changing mode");
        CycleStopGuard cycle(flag);
        check(!cycle, "resuming cycle cannot run while cleanup owns the gate");
    }
    {
        CycleStopGuard cycle(flag);
        check(bool(cycle), "cycle resumes after cleanup completes");
        CycleStopGuard worker(flag);
        check(!worker, "stale frozen heartbeat cannot interrupt a running cycle");
        CycleStopGuard in_cycle_stop(flag, false);
        check(bool(in_cycle_stop), "cycle-owned STOP does not reacquire the gate");
    }
    {
        CycleStopGuard next(flag);
        check(bool(next), "all exits release ownership");
    }
    std::atomic<bool> claimed{false}, resume{false};
    std::thread worker([&] {
        CycleStopGuard cleanup(flag);
        claimed.store(true, std::memory_order_release);
        while (!resume.load(std::memory_order_acquire)) std::this_thread::yield();
    });
    while (!claimed.load(std::memory_order_acquire)) std::this_thread::yield();
    {
        CycleStopGuard game(flag);
        check(!game, "worker paused after claim excludes concurrent game work");
    }
    resume.store(true, std::memory_order_release);
    worker.join();
    CycleStopGuard game(flag);
    check(bool(game), "worker release permits next game cycle");
    return FinishTests();
}
