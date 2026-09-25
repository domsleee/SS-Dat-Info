#pragma once
#include <cstddef>

// Pure parsing of the engine's level-path string. Dependency-free so it can be
// unit-tested standalone (`just test_dll`); level_context.hpp needs the game.
namespace levelpath {

// 0x5C is a backslash, written as a number so the line survives heredocs and codegen.
inline bool IsSep(char c) { return c == '/' || c == (char)0x5C; }

inline char Lower(char c) { return (c >= 'A' && c <= 'Z') ? (char)(c + 32) : c; }

// Case-insensitive match of [p, p+n) against lowercase needles. Returns the index, or -1.
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

// Practice is data/levels/Practice/Tracks/Easy/, with only one difficulty:
// area 3, id 9 ("PE").
inline const char* const AREAS[4] = { "forest", "alpine", "village", "practice" };
inline const char* const DIFFS[3] = { "easy", "medium", "hard" };

// Does `path` contain `seg` (lowercase) as a whole path segment? A substring
// match would accept the game's own "Available_Levels/Forest/Soundtracks/x".
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

// The pointer can land on freed-but-committed heap, which does not fault, so
// require the grammar we depend on.
inline bool IsPlausible(const char* s) {
    if (!s || !s[0]) return false;
    return HasSegmentNoCase(s, "levels") && HasSegmentNoCase(s, "tracks");
}

// Area index (0=Forest, 1=Alpine, 2=Village, 3=Practice) from ".../levels/<area>/...", or -1.
// The path's difficulty segment is not trustworthy: Village Hard resolves to
// ".../village/Tracks/easy/...".
inline int AreaFrom(const char* path) {
    if (!path) return -1;
    for (size_t i = 0; path[i]; i++) {
        if (Lower(path[i]) != 'l') continue;
        // "levels" must begin a segment (see HasSegmentNoCase).
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
        // Not a known area; the real one may appear later in the path.
    }
    return -1;
}

// Level id 0..9 from the path's area and the setup object's area/difficulty, or -1.
// Practice skips "Select Environment", so the setup strings can be a stale
// Arcade selection: trust the setup difficulty only when its area matches the
// path's. Practice has only Easy, so it resolves from the path alone.
inline int LevelIdFrom(int pathArea, const char* setupArea, const char* setupDifficulty) {
    if (pathArea < 0 || pathArea > 3) return -1;
    if (pathArea == 3) return 9;
    if (!setupArea || !setupDifficulty) return -1;
    size_t an = 0; while (setupArea[an]) an++;
    size_t dn = 0; while (setupDifficulty[dn]) dn++;
    const int sa = MatchOne(setupArea, an, AREAS, 4);
    const int sd = MatchOne(setupDifficulty, dn, DIFFS, 3);
    if (sa != pathArea || sd < 0) return -1;  // stale, mismatched or unreadable
    return pathArea * 3 + sd;
}

} // namespace levelpath
