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

inline const char* const AREAS[3] = { "forest", "alpine", "village" };
inline const char* const DIFFS[3] = { "easy", "medium", "hard" };

// Is this a plausible level path?
//
// The pointer we dereference can land on freed-but-committed heap, which does
// NOT fault, so "it read without crashing" proves nothing. Require the grammar
// we actually depend on. Rejects transient garbage and unrelated strings.
inline bool IsPlausible(const char* s) {
    if (!s || !s[0]) return false;
    return ContainsNoCase(s, "levels") && ContainsNoCase(s, "tracks");
}

// Area index (0=Forest, 1=Alpine, 2=Village) from ".../levels/<area>/...", or -1.
//
// This is the half of the identity the path answers RELIABLY. The difficulty
// segment is NOT trustworthy: some tracks share the easy/ shadow asset, so
// Village Hard resolves to ".../village/Tracks/easy/...". Hence area only.
inline int AreaFrom(const char* path) {
    if (!path) return -1;
    for (size_t i = 0; path[i]; i++) {
        if (Lower(path[i]) != 'l') continue;
        size_t j = 0;
        const char* w = "levels";
        while (w[j] && path[i + j] && Lower(path[i + j]) == w[j]) j++;
        if (w[j]) continue;
        size_t k = i + j;
        if (!IsSep(path[k])) continue;
        const char* seg = path + k + 1;
        size_t len = 0;
        while (seg[len] && !IsSep(seg[len])) len++;
        int a = MatchOne(seg, len, AREAS, 3);
        if (a >= 0) return a;
        // "levels" appeared but the next segment is not an area (e.g. Practice,
        // Special, or a literal like "Data/Levels/" with nothing after it).
        // Keep scanning: the real one may appear later in the path.
    }
    return -1;
}

} // namespace levelpath
