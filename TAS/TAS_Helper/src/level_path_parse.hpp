#pragma once
#include <cstddef>

// Pure parsing/validation for the engine's level-path string.
//
// Deliberately dependency-free (no windows.h, no shared state, no SEH) so it can
// be compiled and unit-tested standalone — `just test_dll`. The surrounding
// level_scan.hpp cannot be: it needs the game process. Keeping the decisions
// that are actually easy to get wrong (bounds, grammar, which segment is the
// area) in a testable unit is the point.
namespace levelpath {

// Path separator. 0x5C rather than a backslash char literal so the line survives
// heredocs and codegen intact.
inline bool IsSep(char c) { return c == '/' || c == (char)0x5C; }

inline char Lower(char c) { return (c >= 'A' && c <= 'Z') ? (char)(c + 32) : c; }

// Case-insensitive: does `hay` contain `needle` (lowercase, NUL-terminated)?
inline bool ContainsNoCase(const char* hay, const char* needle) {
    if (!hay || !needle) return false;
    for (size_t i = 0; hay[i]; i++) {
        size_t j = 0;
        while (needle[j] && hay[i + j] && Lower(hay[i + j]) == needle[j]) j++;
        if (!needle[j]) return true;
    }
    return false;
}

// Case-insensitive equality of [p, p+n) against one of `count` lowercase needles.
// Returns the index, or -1.
inline int MatchOne(const char* p, size_t n, const char* const* table, int count) {
    for (int t = 0; t < count; t++) {
        const char* w = table[t];
        size_t wl = 0;
        while (w[wl]) wl++;
        if (wl != n) continue;
        bool ok = true;
        for (size_t i = 0; i < n; i++) {
            if (Lower(p[i]) != w[i]) { ok = false; break; }
        }
        if (ok) return t;
    }
    return -1;
}

// Practice (the start menu's practice run) is a first-class level: on disk it
// is data/levels/Practice/Tracks/Easy/ - the exact grammar of the nine race
// tracks, with exactly ONE difficulty. Area index 3, so id = 3*3+0 = 9 ("PE").
inline const char* const AREAS[4] = { "forest", "alpine", "village", "practice" };
inline const char* const DIFFS[3] = { "easy", "medium", "hard" };

// Does `path` contain `seg` (lowercase) as a WHOLE path segment — i.e. bounded
// by separators or by the ends of the string?
//
// Substring matching is not good enough. A bare ContainsNoCase accepts
// "Available_Levels/Forest/Soundtracks/x", because "levels" hides inside
// "Available_Levels" and "tracks" inside "Soundtracks" — a real filename from
// this game's own data directory, so this is not a hypothetical.
inline bool HasSegmentNoCase(const char* path, const char* seg) {
    if (!path || !seg) return false;
    size_t sl = 0;
    while (seg[sl]) sl++;
    for (size_t i = 0; path[i]; i++) {
        if (i != 0 && !IsSep(path[i - 1])) continue;      // must start a segment
        size_t j = 0;
        while (j < sl && path[i + j] && Lower(path[i + j]) == seg[j]) j++;
        if (j != sl) continue;
        char after = path[i + sl];
        if (after == '\0' || IsSep(after)) return true;   // must end one too
    }
    return false;
}

// Is this a plausible level path?
//
// The pointer we dereference can land on freed-but-committed heap, which does
// NOT fault, so "it read without crashing" proves nothing. Require the grammar
// we actually depend on, as SEGMENTS. Rejects transient garbage, unrelated
// strings, and near-misses like "Available_Levels/.../Soundtracks/...".
inline bool IsPlausible(const char* s) {
    if (!s || !s[0]) return false;
    return HasSegmentNoCase(s, "levels") && HasSegmentNoCase(s, "tracks");
}

// Area index (0=Forest, 1=Alpine, 2=Village, 3=Practice) from ".../levels/<area>/...", or -1.
//
// This is the half of the identity the path answers RELIABLY. The difficulty
// segment is NOT trustworthy: some tracks share the easy/ shadow asset, so
// Village Hard resolves to ".../village/Tracks/easy/...". Hence area only.
inline int AreaFrom(const char* path) {
    if (!path) return -1;
    for (size_t i = 0; path[i]; i++) {
        if (Lower(path[i]) != 'l') continue;
        // "levels" must BEGIN a segment, or "Available_Levels/Forest/..." would
        // report Forest. Same reasoning as HasSegmentNoCase.
        if (i != 0 && !IsSep(path[i - 1])) continue;
        size_t j = 0;
        const char* w = "levels";
        while (w[j] && path[i + j] && Lower(path[i + j]) == w[j]) j++;
        if (w[j]) continue;
        size_t k = i + j;
        if (!IsSep(path[k])) continue;
        const char* seg = path + k + 1;
        size_t len = 0;
        while (seg[len] && !IsSep(seg[len])) len++;
        int a = MatchOne(seg, len, AREAS, 4);
        if (a >= 0) return a;
        // "levels" appeared but the next segment is not a known area (e.g.
        // Special, or a literal like "Data/Levels/" with nothing after it).
        // Keep scanning: the real one may appear later in the path.
    }
    return -1;
}

// Combine the two reliable identity halves into a level id 0..9, or -1.
//
//   pathArea  - AreaFrom(level_path). The path names the AREA reliably (even
//               though it points at the shared easy/ shadow.qua, the area
//               segment is correct); it does NOT name the difficulty.
//   setupArea/setupDifficulty - the game-setup object's strings (authoritative
//               for difficulty). But PRACTICE never writes them - it skips the
//               "Select Environment" screen - so they can be a STALE Arcade
//               selection. Trust the setup difficulty only when its area
//               confirms the SAME area the path reports.
//
// Practice (area 3) has only Easy on disk, so it resolves from the path alone -
// exactly the case a stale setup object would otherwise misidentify.
inline int LevelIdFrom(int pathArea, const char* setupArea, const char* setupDifficulty) {
    if (pathArea < 0 || pathArea > 3) return -1;
    if (pathArea == 3) return 9;  // Practice: Tracks/Easy is the only one on disk
    if (!setupArea || !setupDifficulty) return -1;
    size_t an = 0; while (setupArea[an]) an++;
    size_t dn = 0; while (setupDifficulty[dn]) dn++;
    const int sa = MatchOne(setupArea, an, AREAS, 4);
    const int sd = MatchOne(setupDifficulty, dn, DIFFS, 3);
    if (sa != pathArea || sd < 0) return -1;  // setup is stale / mismatched / unreadable
    return pathArea * 3 + sd;
}

} // namespace levelpath
