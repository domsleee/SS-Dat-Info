#pragma once
#include <cstdint>

// Which controller process owns the TAS (DESIGN.md "Controller ownership").
// The OS calls are injected so the tests can fake exits and PID reuse; no
// Windows dependency here.

// Mirrors TAS_OWNER_* / TAS_OWNER_RESULT_* in shared_state.hpp.
inline constexpr uint32_t OWNER_ACQUIRE = 1;
inline constexpr uint32_t OWNER_RELEASE = 2;
inline constexpr uint32_t OWNER_RESULT_OWNED = 1;
inline constexpr uint32_t OWNER_RESULT_RELEASED = 2;
inline constexpr uint32_t OWNER_RESULT_BUSY = 3;
inline constexpr uint32_t OWNER_RESULT_NO_PROCESS = 4;
inline constexpr uint32_t OWNER_RESULT_WRONG_PROCESS = 5;
inline constexpr uint32_t OWNER_RESULT_NOT_OWNER = 6;
inline constexpr uint32_t OWNER_RESULT_BAD_KIND = 7;

enum class OwnerLiveness { Alive, Exited, Unknown };

struct OwnerOs {
    // A handle that can wait on and query pid, or nullptr.
    void* (*open)(uint32_t pid);
    bool (*created)(void* handle, uint64_t* filetime);
    OwnerLiveness (*poll)(void* handle);
    void (*close)(void* handle);
};

class OwnerTracker {
public:
    explicit OwnerTracker(const OwnerOs& os) : os_(os) {}
    ~OwnerTracker() { Drop(); }
    OwnerTracker(const OwnerTracker&) = delete;
    OwnerTracker& operator=(const OwnerTracker&) = delete;

    uint32_t pid() const { return pid_; }
    uint32_t generation() const { return generation_; }

    // Answer one request. A live owner is never displaced; the same process
    // acquiring again keeps its ownership.
    uint32_t Handle(uint32_t kind, uint32_t pid, uint64_t created) {
        if (kind == OWNER_RELEASE) {
            if (!handle_ || pid != pid_) return OWNER_RESULT_NOT_OWNER;
            Drop();
            ++generation_;
            return OWNER_RESULT_RELEASED;
        }
        if (kind != OWNER_ACQUIRE) return OWNER_RESULT_BAD_KIND;
        if (handle_) {
            if (pid == pid_ && created == created_) return OWNER_RESULT_OWNED;
            return OWNER_RESULT_BUSY;
        }
        void* h = os_.open(pid);
        if (!h) return OWNER_RESULT_NO_PROCESS;
        uint64_t actual = 0;
        // Once opened, the handle pins this process object, so a PID reused
        // later cannot pass as the owner. The creation time proves the PID
        // had not already been reused before the open.
        if (!os_.created(h, &actual) || actual != created) {
            os_.close(h);
            return OWNER_RESULT_WRONG_PROCESS;
        }
        handle_ = h;
        pid_ = pid;
        created_ = created;
        ++generation_;
        return OWNER_RESULT_OWNED;
    }

    // True when the owner has gone: exited, or its handle stopped answering
    // (then nothing can tell it is alive, so it counts as gone). Clears the
    // owner; the caller aborts what it was doing.
    bool OwnerGone() {
        if (!handle_) return false;
        if (os_.poll(handle_) == OwnerLiveness::Alive) return false;
        Drop();
        ++generation_;
        return true;
    }

    void Drop() {
        if (handle_) os_.close(handle_);
        handle_ = nullptr;
        pid_ = 0;
        created_ = 0;
    }

private:
    OwnerOs os_;
    void* handle_ = nullptr;
    uint32_t pid_ = 0;
    uint64_t created_ = 0;
    uint32_t generation_ = 0;
};
