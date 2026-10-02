// Unit tests for controller ownership (controller_owner.hpp): a fake OS for
// PID reuse and failed handles, then real child processes for exit detection
// and handle hygiene.

#include "../owner_os_win32.hpp"
#include "check.hpp"
#include <string>

namespace fake {
// One process table entry per pid; the handle is the entry's address.
struct Proc {
    uint64_t created;
    OwnerLiveness state;
    int open_handles;
};
Proc g_procs[8];

void* Open(uint32_t pid) {
    if (pid >= 8 || g_procs[pid].created == 0) return nullptr;
    g_procs[pid].open_handles++;
    return &g_procs[pid];
}
bool Created(void* h, uint64_t* t) {
    *t = static_cast<Proc*>(h)->created;
    return true;
}
OwnerLiveness Poll(void* h) { return static_cast<Proc*>(h)->state; }
void Close(void* h) { static_cast<Proc*>(h)->open_handles--; }
constexpr OwnerOs kOs{&Open, &Created, &Poll, &Close};

void Spawn(uint32_t pid, uint64_t created) {
    g_procs[pid] = {created, OwnerLiveness::Alive, g_procs[pid].open_handles};
}
}  // namespace fake

static void FakeTests() {
    using fake::g_procs;
    fake::Spawn(1, 100);
    fake::Spawn(2, 200);
    {
        OwnerTracker t(fake::kOs);
        check(t.Handle(OWNER_ACQUIRE, 1, 100) == OWNER_RESULT_OWNED, "acquire_by_live_process");
        const uint32_t gen = t.generation();
        check(t.Handle(OWNER_ACQUIRE, 1, 100) == OWNER_RESULT_OWNED && t.generation() == gen,
              "reacquire_by_owner_keeps_generation");
        check(g_procs[1].open_handles == 1, "reacquire_does_not_leak_a_handle");
        check(t.Handle(OWNER_ACQUIRE, 2, 200) == OWNER_RESULT_BUSY && t.pid() == 1,
              "live_owner_is_never_displaced");
        check(t.Handle(OWNER_RELEASE, 2, 200) == OWNER_RESULT_NOT_OWNER && t.pid() == 1,
              "non_owner_cannot_release");
        check(!t.OwnerGone(), "live_owner_is_not_gone");
        g_procs[1].state = OwnerLiveness::Exited;
        check(t.OwnerGone() && t.pid() == 0 && t.generation() == gen + 1, "exit_is_detected");
        check(!t.OwnerGone(), "exit_is_reported_once");
        check(g_procs[1].open_handles == 0, "exit_closes_the_handle");
        check(t.Handle(OWNER_ACQUIRE, 2, 200) == OWNER_RESULT_OWNED && t.pid() == 2,
              "successor_acquires_after_exit");
        check(t.Handle(OWNER_RELEASE, 2, 200) == OWNER_RESULT_RELEASED && t.pid() == 0,
              "owner_releases");
        check(g_procs[2].open_handles == 0, "release_closes_the_handle");
    }
    {
        OwnerTracker t(fake::kOs);
        // pid 1 was reused by a newer process: the request's creation time is stale.
        fake::Spawn(1, 150);
        check(t.Handle(OWNER_ACQUIRE, 1, 100) == OWNER_RESULT_WRONG_PROCESS && t.pid() == 0,
              "reused_pid_is_refused");
        check(g_procs[1].open_handles == 0, "refused_open_is_closed");
        check(t.Handle(OWNER_ACQUIRE, 7, 700) == OWNER_RESULT_NO_PROCESS, "missing_process_is_refused");
        check(t.Handle(9, 1, 150) == OWNER_RESULT_BAD_KIND, "unknown_kind_is_refused");
        check(t.Handle(OWNER_ACQUIRE, 1, 150) == OWNER_RESULT_OWNED, "acquire_after_refusals");
        g_procs[1].state = OwnerLiveness::Unknown;
        check(t.OwnerGone() && t.pid() == 0, "unanswerable_handle_counts_as_gone");
        check(g_procs[1].open_handles == 0, "gone_handle_is_closed");
        fake::Spawn(1, 150);
        check(t.Handle(OWNER_ACQUIRE, 1, 150) == OWNER_RESULT_OWNED, "acquire_for_destructor");
    }
    check(g_procs[1].open_handles == 0, "destructor_closes_the_handle");
}

static PROCESS_INFORMATION SpawnSleeper() {
    char exe[MAX_PATH];
    GetModuleFileNameA(nullptr, exe, MAX_PATH);
    std::string command = std::string("\"") + exe + "\" sleep";
    STARTUPINFOA startup = {};
    startup.cb = sizeof(startup);
    PROCESS_INFORMATION process = {};
    CreateProcessA(nullptr, command.data(), nullptr, nullptr, FALSE, CREATE_NO_WINDOW,
                   nullptr, nullptr, &startup, &process);
    return process;
}

static void RealProcessTests() {
    PROCESS_INFORMATION child = SpawnSleeper();
    check(child.hProcess != nullptr, "child_started");
    uint64_t created = 0;
    owner_os::Created(child.hProcess, &created);
    OwnerTracker t(owner_os::kWin32);
    check(t.Handle(OWNER_ACQUIRE, child.dwProcessId, created + 1) == OWNER_RESULT_WRONG_PROCESS,
          "real_wrong_creation_time_is_refused");
    check(t.Handle(OWNER_ACQUIRE, child.dwProcessId, created) == OWNER_RESULT_OWNED,
          "real_child_acquires");
    check(!t.OwnerGone(), "real_live_child_is_not_gone");
    TerminateProcess(child.hProcess, 1);
    WaitForSingleObject(child.hProcess, 5000);
    check(t.OwnerGone(), "real_killed_child_is_gone");
    CloseHandle(child.hThread);
    CloseHandle(child.hProcess);

    // Own process: acquire and release many times without growing the handle table.
    uint64_t self_created = 0;
    owner_os::Created(GetCurrentProcess(), &self_created);
    DWORD before = 0, after = 0;
    GetProcessHandleCount(GetCurrentProcess(), &before);
    bool all_ok = true;
    for (int i = 0; i < 1000; i++) {
        all_ok &= t.Handle(OWNER_ACQUIRE, GetCurrentProcessId(), self_created) == OWNER_RESULT_OWNED;
        all_ok &= t.Handle(OWNER_RELEASE, GetCurrentProcessId(), 0) == OWNER_RESULT_RELEASED;
    }
    GetProcessHandleCount(GetCurrentProcess(), &after);
    check(all_ok && after <= before + 1, "repeated_acquire_release_keeps_handle_count_flat");
}

int main(int argc, char** argv) {
    if (argc == 2 && std::string(argv[1]) == "sleep") {
        Sleep(30000);
        return 0;
    }
    std::printf("controller_owner tests:\n");
    FakeTests();
    RealProcessTests();
    return FinishTests();
}
