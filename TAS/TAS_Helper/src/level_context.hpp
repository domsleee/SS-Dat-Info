#pragma once
#include <windows.h>
#include <cstdint>
#include <cstddef>
#include <cstring>
#include "shared_state.hpp"
#include "log.hpp"
#include "setup_object.hpp"
#include "level_path_parse.hpp"

// Publishes which track is running (level_id, level_path; see shared_state.hpp).
// caves/lifecycle_cave.hpp calls these from the game's launch and stop points.
// The area comes from the level path ([SG+0x1D3304]), the difficulty from the
// game-setup object, since tracks can share a path.
namespace levelcontext {

inline uint32_t g_levelPathPtrAddr = 0;
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// SEH only contains faults; freed heap reads fine and can be rewritten
// mid-copy. So require a NUL in bounds and a plausible grammar here, and
// readLevelPath requires two agreeing samples.
static bool readLevelPathOnce(char* out, size_t cap) {
    if (!g_levelPathPtrAddr || !g_readPtr) return false;
    uint32_t p = g_readPtr(g_levelPathPtrAddr);
    if (!p) return false;
    __try {
        const char* src = (const char*)p;
        size_t n = 0;
        while (n < cap && src[n]) n++;
        if (n == 0 || n >= cap) return false;   // no NUL in bounds => not a path
        for (size_t i = 0; i < n; i++) out[i] = src[i];
        out[n] = '\0';
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    return levelpath::IsPlausible(out);
}

static bool readLevelPath(char* out, size_t cap) {
    char a[TAS_LEVEL_PATH_MAX];
    if (!readLevelPathOnce(a, sizeof(a))) return false;
    char b[TAS_LEVEL_PATH_MAX];
    if (!readLevelPathOnce(b, sizeof(b))) return false;
    if (std::strcmp(a, b) != 0) return false;   // torn / mid-rewrite
    size_t n = 0;
    while (n < cap - 1 && a[n]) { out[n] = a[n]; n++; }
    out[n] = '\0';
    return true;
}

// Seqlock writer. Interlocked increments are full barriers. Must not nest: an
// inner pair would make the sequence even mid-write.
template <typename F>
static void publishContext(TasSharedState* s, F&& fn) {
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> odd: writing
    fn();
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> even: stable
}

// Returns area*3+diff (Practice = 9) or -1; see levelpath::LevelIdFrom.
static int32_t identify(const char* path) {
    const int area = levelpath::AreaFrom(path);
    if (area < 0) return -1;
    gamesetup::Setup setup;
    gamesetup::Read(&setup);
    return levelpath::LevelIdFrom(area, setup.area, setup.difficulty);
}

// A race is running: publish a new context, unresolved if the track was not
// identified (never as "no race").
inline void PublishRunning(TasSharedState* s) {
    char path[TAS_LEVEL_PATH_MAX] = {};
    const bool havePath = readLevelPath(path, sizeof(path));
    const int32_t id = havePath ? identify(path) : -1;
    publishContext(s, [&] {
        for (size_t i = 0; i < TAS_LEVEL_PATH_MAX; i++) s->level_path[i] = path[i];
        s->level_path_gen++;
        s->level_epoch++;
        s->level_id = id >= 0 ? (uint32_t)id : 0xFFFFFFFFu;
        s->level_scan_epoch = id >= 0 ? s->level_epoch : s->level_epoch - 1u;
    });
    Log(std::format("Level: racing '{}' (id {})", path, id));
}

// Retry identifying an unresolved race, in the same context (no epoch bump).
inline void RetryIfUnresolved(TasSharedState* s) {
    if (s->level_scan_epoch == s->level_epoch) return;
    char path[TAS_LEVEL_PATH_MAX] = {};
    if (!readLevelPath(path, sizeof(path))) return;
    const int32_t id = identify(path);
    if (id < 0) return;
    publishContext(s, [&] {
        s->level_id = (uint32_t)id;
        s->level_scan_epoch = s->level_epoch;
    });
    Log(std::format("Level: identified '{}' (id {}) after launch", path, id));
}

// The race was left (menu, track switch): resolved, no level.
inline void PublishLeft(TasSharedState* s) {
    publishContext(s, [&] {
        s->level_path[0] = '\0';
        s->level_path_gen++;
        s->level_epoch++;
        s->level_id = 0xFFFFFFFFu;
        s->level_scan_epoch = s->level_epoch;
    });
}

// Starts as "no race"; if injected mid-race, the first Supreme::Cycle tick
// corrects it (lifecycle_cave.hpp).
inline void Init(TasSharedState* s, uint32_t levelPathPtrAddr, uint32_t (*readPtr)(uint32_t)) {
    g_levelPathPtrAddr = levelPathPtrAddr;
    g_readPtr = readPtr;
    // Shared memory survives reinjection; a killed instance can leave the
    // sequence odd, which would reject every read.
    if (InterlockedOr((volatile LONG*)&s->level_ctx_seq, 0) & 1) {
        InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);
    }
    PublishLeft(s);
}

} // namespace levelcontext
