#pragma once

#include <windows.h>
#include <cstdint>
#include <safetyhook.hpp>
#include "Log.hpp"

namespace f7 {
    static HMODULE g_module = nullptr;
    using SetMode = bool(__fastcall*)(void*);
    using GetSupreme = void* (*)();
    static SetMode g_setAiLearningMode = nullptr;
    static SetMode g_setReplayMode = nullptr;
    static GetSupreme g_getSupreme = nullptr;

    static void* g_handler = nullptr;
    static uint32_t g_handlerRoot = 0;
    static safetyhook::MidHook g_setControllerHook;

    static uint32_t LevelRoot() {
        __try {
            return *(uint32_t*)((uint8_t*)g_module + 0x1D5450);
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
    }

    static void Install(HMODULE module) {
        g_module = module;
        auto base = (uint8_t*)module;
        g_setAiLearningMode = (SetMode)(base + 0x142130);
        g_setReplayMode = (SetMode)(base + 0x1417E0);
        g_getSupreme = (GetSupreme)(base + 0x13E410);
        // Set_Controller runs at level start and F5. Stamp the captured
        // handler so a changed level root invalidates it before any read.
        g_setControllerHook = safetyhook::create_mid(base + 0x942F0,
            [](safetyhook::Context& ctx) {
                g_handler = (void*)ctx.ecx;
                g_handlerRoot = LevelRoot();
            });
        Log("DoCustomInput: Player_Handler::Set_Controller hook installed");
    }

    // 1: guide/TOP5 or finished-run ghost exists; 0: empty; -1: unavailable.
    // Starting replay with an empty list causes a fatal error on a later tick.
    static int GhostsLoaded() {
        constexpr size_t GHOST_LIST_OFFSET = 0x20;
        constexpr size_t NODE_OFFSET = 0x4;
        constexpr size_t NODE_CHILD_HEAD = 0xC;
        constexpr size_t NODE_CHILD_TAIL = 0x10;
        __try {
            auto handler = (uint8_t*)g_handler;
            if (!handler) return -1;
            const uint32_t root = LevelRoot();
            if (!root || root != g_handlerRoot) return -1;
            auto list = *(uint8_t**)(handler + GHOST_LIST_OFFSET);
            if (!list) return -1;
            auto node = list + NODE_OFFSET;
            auto head = *(uint8_t**)(node + NODE_CHILD_HEAD);
            if (!head) return -1;
            // An empty FLIT list points its head at its own tail sentinel.
            return head != node + NODE_CHILD_TAIL ? 1 : 0;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return -1;
        }
    }

    // Supreme.exe+0x30E0 constructs these Arcade Game subclasses. They share
    // the results flag at +0x10, cleared by Game::Reset (exe+0x24F40).
    // Returns 1 if cleared, 0 if already hidden, -1 for an unavailable object.
    static int ClearResultsFlag() {
        constexpr uintptr_t GAME_VTABLE_RVAS[] = {
            0x6D6EC, // Time Attack
            0x6D730, // Race
            0x6D6A8, // Pipe
            0x6D664, // Air
        };
        __try {
            auto exe = (uint8_t*)GetModuleHandleA(nullptr);
            auto app = *(uint8_t**)(exe + 0x889C4);
            if (!app) return -1;
            auto game = *(uint8_t**)(app + 0x30);
            if (!game) return -1;
            auto vtable = *(uint8_t**)game;
            for (const auto rva : GAME_VTABLE_RVAS) {
                if (vtable != exe + rva) continue;
                if (!game[0x10]) return 0;
                game[0x10] = 0;
                return 1;
            }
            return -1;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return -1;
        }
    }

    // MSVC6 std::string is {allocator, buffer, length, capacity}. Return the
    // game-owned prompt only when readable, nonempty, and consistent.
    static void* PromptStringHoldingText() {
        __try {
            auto startInfo = *(uint8_t**)((uint8_t*)g_getSupreme() + 0x34);
            if (!startInfo) return nullptr;
            auto str = (uint32_t*)(startInfo + 0xD4);
            auto text = (const char*)str[1];
            const uint32_t len = str[2];
            if (!text || len == 0 || len > 256) return nullptr;
            size_t actual = 0;
            while (actual < len && text[actual]) actual++;
            return actual == len ? str : nullptr;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return nullptr;
        }
    }

    // Use the game's runtime: MSVC6 strings can share reference-counted
    // buffers. The dummy EDX adapts fastcall to thiscall's stack arguments.
    using StringAssign = void* (__fastcall*)(void*, void*, const char*, unsigned);
    static StringAssign ResolveStringAssign() {
        static StringAssign assign = nullptr;
        if (assign) return assign;
        HMODULE msvcp = GetModuleHandleA("MSVCP60.dll");
        if (!msvcp) {
            Log("F7: MSVCP60.dll not loaded - replay instructions line left as is");
            return nullptr;
        }
        assign = (StringAssign)GetProcAddress(msvcp,
            "?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEAAV12@PBDI@Z");
        if (!assign) Log("F7: MSVCP60 basic_string::assign(const char*, size) export not found");
        return assign;
    }

    static void HideResults() {
        const int page = ClearResultsFlag();
        if (page == 1) Log("F7: results page hidden");
        if (page == -1) Log("F7: exe Game object not found - results page left as is");

        void* prompt = PromptStringHoldingText();
        if (!prompt) return;
        StringAssign assign = ResolveStringAssign();
        if (!assign) return;
        assign(prompt, nullptr, "", 0);
        Log("F7: replay instructions line blanked");
    }

    static void Handle(void* supreme) {
        Log("Handle F7");
        const int ghosts = GhostsLoaded();
        if (ghosts != 1) {
            Log(ghosts == 0 ? "F7: nothing to replay (no ghost loaded, no finished run) - ignored"
                            : "F7: player handler not seen yet - ignored");
            return;
        }
        g_setAiLearningMode(supreme);
        g_setReplayMode(supreme);
        HideResults();

        // Preserve F7's existing camera toggle (the game's C key).
        auto camera = *(uint8_t**)((uint8_t*)g_getSupreme() + 0x144);
        if (camera) {
            auto& alternate = *(int*)(camera + 0x68);
            alternate = alternate == 0;
        }
    }
}
