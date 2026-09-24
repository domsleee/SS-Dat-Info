#pragma once
#include <windows.h>
#include <cstdint>
#include <cstddef>
#include <cstring>
#include "shared_state.hpp"
#include "log.hpp"
#include "setup_object.hpp"
#include "level_path_parse.hpp"

// The level context: which track is running, published when the game launches
// a race and cleared when it leaves one (caves/lifecycle.hpp calls these from
// the game's own launch and stop points).
//
// level_id: 0..9 = area*3 + difficulty (area 0=Forest, 1=Alpine, 2=Village,
// 3=Practice; difficulty 0=Easy, 1=Medium, 2=Hard; Practice has only Easy, so
// only 9 occurs); 0xFFFFFFFF = no race / unknown.
//
// The area comes from the level's resource path ([SG+0x1D3304]), the
// difficulty from the game-setup object: some tracks share a path (Village
// Hard loads ".../Tracks/easy/..."), so the path alone cannot tell them apart.
namespace levelcontext {

inline uint32_t g_levelPathPtrAddr = 0;
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// Read the current level path into `out`. Returns false when unavailable OR not
// a plausible level path.
//
// SEH only contains crashes; it does not give a coherent snapshot. Freed-but-
// still-committed heap does not fault, and an in-place rewrite between the
// length walk and the copy yields a hybrid string. So:
//   * require a NUL WITHIN bounds (127 non-NUL bytes is not a valid path, it is
//     a garbage buffer that happened to be readable);
//   * require the path grammar we actually depend on ("levels" and "tracks"),
//     which rejects transient garbage and unrelated strings;
//   * sample TWICE and require the two to agree, which rejects a torn read.
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
    // Grammar check — see levelpath::IsPlausible, unit-tested standalone.
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

// Seqlock writer. Marks the group as being mutated, applies `fn`, then commits.
//
// InterlockedIncrement rather than `++` so the sequence transitions are full
// barriers that neither the compiler nor the CPU can reorder around the
// payload. MUST NOT NEST: an inner pair would drive the sequence back to EVEN
// halfway through the outer write, publishing a torn group as if it were stable.
template <typename F>
static void publishContext(TasSharedState* s, F&& fn) {
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> odd: writing
    fn();
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> even: stable
}

// Area from the path, difficulty from the setup object. Practice resolves from
// the path alone: it skips the menu screen that writes the setup object, so the
// object holds a stale Arcade selection there (levelpath::LevelIdFrom).
// Returns area*3+diff (Practice = 9) or -1.
static int32_t identify(const char* path) {
    const int area = levelpath::AreaFrom(path);
    if (area < 0) return -1;
    gamesetup::Setup setup;
    gamesetup::Read(&setup);
    return levelpath::LevelIdFrom(area, setup.area, setup.difficulty);
}

// A race is running: identify it and publish it as one resolved context.
inline void PublishRunning(TasSharedState* s) {
    char path[TAS_LEVEL_PATH_MAX] = {};
    const bool havePath = readLevelPath(path, sizeof(path));
    const int32_t id = havePath ? identify(path) : -1;
    publishContext(s, [&] {
        for (size_t i = 0; i < TAS_LEVEL_PATH_MAX; i++) s->level_path[i] = path[i];
        s->level_path_gen++;
        s->level_epoch++;
        s->level_id = id >= 0 ? (uint32_t)id : 0xFFFFFFFFu;
        s->level_scan_epoch = s->level_epoch;   // resolved
    });
    Log(std::format("Level: racing '{}' (id {})", path, id));
}

// The race was left (quit to the menu, a track switch): resolved, no level.
inline void PublishLeft(TasSharedState* s) {
    publishContext(s, [&] {
        s->level_path[0] = '\0';
        s->level_path_gen++;
        s->level_epoch++;
        s->level_id = 0xFFFFFFFFu;
        s->level_scan_epoch = s->level_epoch;   // resolved: no race
    });
}

// At install: no race. Injected into a running race, the first Supreme::Cycle
// tick corrects this (lifecycle.hpp).
inline void Init(TasSharedState* s, uint32_t levelPathPtrAddr, uint32_t (*readPtr)(uint32_t)) {
    g_levelPathPtrAddr = levelPathPtrAddr;
    g_readPtr = readPtr;
    // Shared memory survives reinjection, so a previous DLL instance killed
    // mid-write can leave the sequence ODD, which rejects every read forever.
    // Nothing is in flight here, so re-establish an even sequence.
    if (InterlockedOr((volatile LONG*)&s->level_ctx_seq, 0) & 1) {
        InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);
    }
    PublishLeft(s);
}

} // namespace levelcontext
