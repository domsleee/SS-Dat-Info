#pragma once
// Shared by the standalone suites in this directory (one exe per test_*.cpp,
// built and run by TAS/tools/test_dll_hidden.ps1).
#include <cstdio>

inline int g_failures = 0;

inline void check(bool cond, const char* name) {
    std::printf("  %-4s %s\n", cond ? "ok" : "FAIL", name);
    if (!cond) g_failures++;
}

// Prints the verdict; main() returns it.
inline int FinishTests() {
    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
