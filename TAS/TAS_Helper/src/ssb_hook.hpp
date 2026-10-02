#pragma once
#include "stdafx.h"
#include "log.hpp"
#include <safetyhook.hpp>
#include <vector>

// ssb_hook.dll (ssb-recomp crates/ssb-hook): the Rust ports of game functions.
// Loaded at init when it sits next to TAS_Helper.dll, so its detours are live
// before any race tick. The Rust side only lists its detours; SafetyHook inline
// hooks install them, so one hooking library patches the game. All or nothing:
// on any failure no detour stays installed.
namespace ssbhook {

// Mirrors crates/ssb-hook/src/lib.rs.
struct Detour {
    const char* name;
    const char* module;
    uint32_t rva;
    const uint8_t* prologue;   // expected first bytes of the original
    uint32_t prologue_len;
    void* replacement;
    uintptr_t* original;       // receives the trampoline before the hook goes live
};
struct Host {
    uint32_t version;
    void(__cdecl* log)(const char*);
    void(__cdecl* flush_trace)();
    uint32_t expect_cw;      // the x87 control word every detour entry must see (ssb_trace_env.txt)
    uint32_t expect_mxcsr;   // MXCSR control bits (the low six sticky flags are not compared)
};
static_assert(sizeof(Detour) == 28 && sizeof(Host) == 20);
constexpr uint32_t ABI_VERSION = 2;
using InitFn = int(__cdecl*)(const Host*, const Detour**, uint32_t*);
using CallsFn = uint32_t(__cdecl*)();

inline CallsFn g_calls = nullptr;
inline CallsFn g_violations = nullptr;
inline std::vector<SafetyHookInline> g_hooks;
inline std::string g_summary = "none";  // for trace headers
inline Host g_host{};

// ssb-hook detour calls so far; 0 without it.
inline uint32_t Calls() { return g_calls ? g_calls() : 0; }
// Detour entries that saw an FPU environment other than the expected one;
// latched by ssb-hook, recorded per tick, fatal to the checker.
inline uint32_t FpuViolations() { return g_violations ? g_violations() : 0; }

// The folder holding TAS_Helper.dll, with a trailing backslash.
inline std::string DllDir() {
    HMODULE self = nullptr;
    char path[MAX_PATH] = {};
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                       reinterpret_cast<LPCSTR>(&DllDir), &self);
    GetModuleFileNameA(self, path, MAX_PATH);
    std::string s(path);
    return s.substr(0, s.find_last_of("\\/") + 1);
}

static void __cdecl HostLog(const char* m) { Log(std::string("ssb-hook: ") + (m ? m : "")); }

inline void Unload() {
    g_hooks.clear();   // each destructor restores the original bytes
    g_calls = nullptr;
    g_violations = nullptr;
}

// False only when ssb_hook.dll is present and could not be installed. It
// needs the expected FPU environment (`haveEnv`: ssb_trace_env.txt).
inline bool Load(void(__cdecl* flushTrace)(), bool haveEnv, uint16_t cw, uint32_t mxcsr) {
    const std::string path = DllDir() + "ssb_hook.dll";
    if (GetFileAttributesA(path.c_str()) == INVALID_FILE_ATTRIBUTES) return true;
    if (!haveEnv) {
        Log("ssb-hook: ssb_hook.dll needs ssb_trace_env.txt (the expected FPU environment)");
        return false;
    }

    HMODULE mod = LoadLibraryA(path.c_str());
    auto init = mod ? (InitFn)GetProcAddress(mod, "ssb_hook_init") : nullptr;
    auto calls = mod ? (CallsFn)GetProcAddress(mod, "ssb_hook_calls") : nullptr;
    auto violations = mod ? (CallsFn)GetProcAddress(mod, "ssb_hook_fpu_violations") : nullptr;
    if (!init || !calls || !violations) {
        Log(std::format("ssb-hook: FAILED to load {} (error {})", path, GetLastError()));
        return false;
    }
    g_host = {ABI_VERSION, &HostLog, flushTrace, cw, mxcsr};
    const Detour* table = nullptr;
    uint32_t count = 0;
    if (init(&g_host, &table, &count) != 0 || (count && !table)) {
        Log("ssb-hook: ssb_hook_init refused (ABI mismatch?)");
        return false;
    }

    std::string summary = path;
    for (uint32_t i = 0; i < count; i++) {
        const Detour& d = table[i];
        auto base = (uint8_t*)GetModuleHandleA(d.module);
        uint8_t* target = base ? base + d.rva : nullptr;
        if (!target || std::memcmp(target, d.prologue, d.prologue_len) != 0) {
            Log(std::format("ssb-hook: detour {} at {}+{:#x}: module missing or prologue differs; none installed",
                            d.name, d.module, d.rva));
            Unload();
            return false;
        }
        auto hook = safetyhook::create_inline(target, d.replacement, SafetyHookInline::StartDisabled);
        if (!hook) {
            Log(std::format("ssb-hook: detour {} could not be created; none installed", d.name));
            Unload();
            return false;
        }
        *d.original = reinterpret_cast<uintptr_t>(hook.original<void*>());
        g_hooks.push_back(std::move(hook));
        summary += std::format(" | {} {}+{:#x}", d.name, d.module, d.rva);
    }
    for (auto& h : g_hooks) {
        if (!h.enable()) {
            Log("ssb-hook: enabling a detour failed; none installed");
            Unload();
            return false;
        }
    }
    g_calls = calls;
    g_violations = violations;
    g_summary = summary;
    Log(std::format("ssb-hook: loaded, {} detour(s): {}", count, summary));
    return true;
}

} // namespace ssbhook
