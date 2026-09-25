#pragma once
#include "stdafx.h"
#include "log.hpp"

// Game module bases and resolved addresses. Every offset is a fixed RVA of the
// stock v1.035 images, pinned by the checks in Resolve.

// Housemarque Kernel::Time: 64-bit timestamp ({lo, hi}; QPC-derived uptime units).
// Key events are stamped at message-pump dispatch: Win32_Driver::Translate
// → keyDown/keyUp (+3940/+3980) → BB3B10(keyIndex, pressed, Time.lo, Time.hi).
// The observer drops events stamped before the current race context.
struct KernelTime { uint32_t lo; uint32_t hi; };

// ?Current@Time@Kernel@Housemarque@@SI?AV123@XZ: static __fastcall, returns
// the Time through a hidden return slot in ecx.
using KernelTimeCurrentFn = KernelTime*(__fastcall*)(KernelTime* out, void* edx_unused);

struct GameAddresses {
    struct ModuleIdentity {
        const char* name;
        uint32_t timestamp;
        uint32_t image_size;
    };

    static bool ValidateModule(HMODULE module, const ModuleIdentity& expected) {
        if (!module) {
            Log(std::format("ERROR: {} is not loaded", expected.name));
            return false;
        }

        auto* base = reinterpret_cast<std::uint8_t*>(module);
        auto* dos = reinterpret_cast<PIMAGE_DOS_HEADER>(base);
        if (dos->e_magic != IMAGE_DOS_SIGNATURE) {
            Log(std::format("ERROR: {} has an invalid DOS header", expected.name));
            return false;
        }
        auto* nt = reinterpret_cast<PIMAGE_NT_HEADERS>(base + dos->e_lfanew);
        if (nt->Signature != IMAGE_NT_SIGNATURE) {
            Log(std::format("ERROR: {} has an invalid PE header", expected.name));
            return false;
        }

        const uint32_t timestamp = nt->FileHeader.TimeDateStamp;
        const uint32_t imageSize = nt->OptionalHeader.SizeOfImage;
        if (timestamp != expected.timestamp || imageSize != expected.image_size) {
            Log(std::format(
                "ERROR: unsupported {} build (timestamp=0x{:08X}, image_size=0x{:X}; "
                "expected 0x{:08X}/0x{:X})",
                expected.name, timestamp, imageSize, expected.timestamp, expected.image_size));
            return false;
        }
        return true;
    }

    static std::string HexBytes(const std::uint8_t* p, size_t n) {
        std::string s;
        for (size_t i = 0; i < n; i++) s += std::format("{}{:02X}", i ? " " : "", p[i]);
        return s;
    }

    static bool ValidateCodeBytes(const char* label, const std::uint8_t* address,
                                  const std::uint8_t* expected, size_t n) {
        if (std::memcmp(address, expected, n) == 0) return true;
        Log(std::format("ERROR: unsupported game build: {} bytes do not match (live [{}], expected [{}])",
                        label, HexBytes(address, n), HexBytes(expected, n)));
        return false;
    }

    // Patterns must not contain absolute-address immediates: Supreme_Game,
    // HMG_Cetsup_Win32 and SR_UIT are always relocated. Use ValidateCodeAbs for those.
    template <size_t N>
    static bool ValidateCode(const char* label, const std::uint8_t* address,
                             const std::uint8_t (&expected)[N]) {
        return ValidateCodeBytes(label, address, expected, N);
    }

    // Compare live code whose imm32 at `AbsOffset` is a relocated absolute
    // address: it is rewritten to `moduleBase + absRva` before comparing.
    template <size_t AbsOffset, size_t N>
    static bool ValidateCodeAbs(const char* label, const std::uint8_t* address,
                                const std::uint8_t (&expected)[N],
                                const std::uint8_t* moduleBase, uint32_t absRva) {
        static_assert(AbsOffset + sizeof(uint32_t) <= N, "imm32 must lie inside the pattern");
        std::uint8_t rebased[N];
        std::memcpy(rebased, expected, N);
        const uint32_t abs = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(moduleBase + absRva));
        std::memcpy(rebased + AbsOffset, &abs, sizeof(abs));
        return ValidateCodeBytes(label, address, rebased, N);
    }

    // Like ValidateCode, but also accepts a JMP (E9): Display_Config_Helper is
    // injected first and inline-hooks the two HMG key handlers for its F5
    // debounce. SafetyHook chains onto that hook.
    template <size_t N>
    static bool ValidateCodeOrHooked(const char* label, const std::uint8_t* address,
                                     const std::uint8_t (&expected)[N]) {
        if (address[0] == 0xE9) {
            Log(std::format("{}: already inline-hooked by another module ({}); chaining",
                            label, HexBytes(address, 5)));
            return true;
        }
        return ValidateCodeBytes(label, address, expected, N);
    }

    HMODULE exe = nullptr;   // Supreme.exe
    HMODULE sg = nullptr;    // Supreme_Game.dll
    HMODULE hmg = nullptr;   // HMG_Cetsup_Win32.dll
    HMODULE kernel = nullptr; // HMG_Kernel.dll

    // Stamps injected BB3B10 events like a real keypress. nullptr = export
    // missing (fall back to calibrated arg4).
    KernelTimeCurrentFn time_current = nullptr;

    std::uint8_t* tick_cave_site = nullptr;     // exe+0x25C81: after ftol+mov esi,eax (tick override)
    std::uint8_t* f5_accept_site = nullptr; // exe+0x25C3F: F5 down and accepted, restart call next
    std::uint8_t* launch_site = nullptr;    // exe+0x25BD7: race loop entered the level, game mode set
    std::uint8_t* pump_site = nullptr;      // exe+0x55920: the game's message pump (runs in every state)

    std::uint8_t* cycle_cave_site = nullptr;     // SG+0x13FE40: Supreme::Cycle
    std::uint8_t* replay_capture_site = nullptr; // SG+0x9E8F0: replay object capture
    std::uint8_t* f5_done_site = nullptr;   // SG+0x14199F: Set_Game_Mode's mode init returned
    std::uint8_t* stop_site = nullptr;      // SG+0x1408F0: Supreme::Stop (the race is being left)
    std::uint8_t* player_base = nullptr;    // SG+0x1D5450: root pointer
    // Live vtables the replay-capture hook classifies recorder owners with;
    // 0 until Resolve validated the constructor sites.
    std::uint32_t player_vtable = 0;
    std::uint32_t ghost_vtable = 0;
    // SG+0x1D3304: pointer to the current level's resource path. Reliable for
    // area, not difficulty: Village Hard reads ".../Tracks/easy/...".
    std::uint8_t* level_path_ptr = nullptr;

    std::uint8_t* key_down_site = nullptr;    // HMG+0x3940: key handler (down)
    std::uint8_t* key_up_site = nullptr;      // HMG+0x3980: key handler (up)
    std::uint8_t* bb3b10 = nullptr;         // HMG+0x3B10: BB3B10 observer

    // Game key codes: both the DI-buffer index and BB3B10's keyIndex.
    static constexpr uint32_t KEY_UP     = 0x38;
    static constexpr uint32_t KEY_DOWN   = 0x39;
    static constexpr uint32_t KEY_LEFT   = 0x3A;
    static constexpr uint32_t KEY_RIGHT  = 0x3B;
    static constexpr uint32_t KEY_JUMP   = 0x27;  // LCTRL
    static constexpr uint32_t KEY_JUMP2  = 0x28;  // CTRL (duplicate)
    static constexpr uint32_t KEY_SHIFT  = 0x24;  // SHIFT
    static constexpr uint32_t KEY_SHIFT2 = 0x25;  // LSHIFT (duplicate)
    static constexpr uint32_t KEY_F5     = 0x58;  // restart race
    // The pause menu hears ESC through the BB3B10 broadcast, so the REC-mode
    // observer block must exempt it.
    static constexpr uint32_t KEY_ESC    = 0x48;

    // root = [SG+1D5450], kbobj = [root+530], buffer = [kbobj+30]
    static constexpr uint32_t ROOT_PTR_OFFSET = 0x1D5450;
    static constexpr uint32_t LEVEL_PATH_PTR_OFFSET = 0x1D3304;
    static constexpr uint32_t KEYBOARD_OBJ_OFFSET = 0x530;
    static constexpr uint32_t DI_BUFFER_PTR_OFFSET = 0x30;
    // kbobj is a 0x38-byte Win32_Keyboard. The 256-byte buffer at +0x30 is the
    // only key state the DLL writes; Win32_Keyboard::State reads it.

    static constexpr uint32_t REPLAY_PLAYER_OFFSET = 0x84;
    // The human rider is a plain `Player`; ghosts are `Ghost_Player`, others
    // `AI_Player` / `Net_Player`. A Player links back to its recorder at +0x14C
    // (see replay_identity.hpp). Resolve validates both vtable RVAs against the
    // constructors' `mov [this], offset vtable`.
    // Loadout object [player+0x20] (no RTTI) holds MSVC6 std::strings
    // ({allocator, ptr, size, capacity}, 16 bytes): character folder at +0x10
    // ("vincent"), character config path at +0x30, board config path at +0x60
    // (no effect on physics). Player_Config [player+0x48] has the display name
    // at +0x48 ("Vincent").
    static constexpr uint32_t PLAYER_LOADOUT_OFFSET = 0x20;
    static constexpr uint32_t PLAYER_CONFIG_OFFSET = 0x48;
    static constexpr uint32_t LOADOUT_FOLDER_STRING = 0x10;
    static constexpr uint32_t PLAYER_CONFIG_NAME_STRING = 0x48;
    static constexpr uint32_t MSVC6_STRING_PTR = 0x4;
    static constexpr uint32_t MSVC6_STRING_SIZE = 0x8;
    // Stance, area and difficulty come from the setup object (setup_object.hpp).
    static constexpr uint32_t PLAYER_VTABLE_RVA = 0x169E10;        // .?AVPlayer@Supreme_Snowboarding@Housemarque@@
    static constexpr uint32_t GHOST_PLAYER_VTABLE_RVA = 0x169B74;  // .?AVGhost_Player@...
    static constexpr uint32_t PLAYER_X = 0xF8;
    static constexpr uint32_t PLAYER_Y = 0xFC;
    static constexpr uint32_t PLAYER_Z = 0x100;

    // Fallback Time.hi when Kernel::Time::Current is unavailable: what a real
    // steering keypress passed in a live capture.
    static constexpr uint32_t BB3B10_ARG4 = 0x96;
    // BB3B10's this = kbobj + 0x18.
    static constexpr uint32_t BB3B10_THIS_OFFSET = 0x18;

    bool Resolve() {
        exe = GetModuleHandleA(nullptr);
        sg = GetModuleHandleA("Supreme_Game.dll");
        hmg = GetModuleHandleA("HMG_Cetsup_Win32.dll");
        kernel = GetModuleHandleA("HMG_Kernel.dll");

        if (!exe) { Log("ERROR: Supreme.exe not found"); return false; }
        if (!sg) { Log("ERROR: Supreme_Game.dll not loaded"); return false; }
        if (!hmg) { Log("ERROR: HMG_Cetsup_Win32.dll not loaded"); return false; }

        // Refuse any other build: the RVAs below are for v1.035 only.
        static constexpr ModuleIdentity kExeIdentity{
            "Supreme.exe v1.035", 0x381DA4A3u, 0x0008A000u
        };
        static constexpr ModuleIdentity kSgIdentity{
            "Supreme_Game.dll v1.035", 0x381DA46Eu, 0x001FE000u
        };
        static constexpr ModuleIdentity kHmgIdentity{
            "HMG_Cetsup_Win32.dll v1.035", 0x3811D8D7u, 0x0000B000u
        };
        if (!ValidateModule(exe, kExeIdentity) ||
            !ValidateModule(sg, kSgIdentity) ||
            !ValidateModule(hmg, kHmgIdentity)) {
            return false;
        }

        // Non-fatal if missing: injection falls back to the calibrated arg4.
        if (kernel) {
            time_current = (KernelTimeCurrentFn)GetProcAddress(
                kernel, "?Current@Time@Kernel@Housemarque@@SI?AV123@XZ");
        }
        if (!time_current) {
            Log("WARNING: Kernel::Time::Current not resolved — injected input "
                "falls back to calibrated arg4");
        }

        auto exeBase = (std::uint8_t*)exe;
        auto sgBase = (std::uint8_t*)sg;
        auto hmgBase = (std::uint8_t*)hmg;

        tick_cave_site = exeBase + 0x25C81;
        f5_accept_site = exeBase + 0x25C3F;
        launch_site = exeBase + 0x25BD7;
        pump_site = exeBase + 0x55920;

        cycle_cave_site = sgBase + 0x13FE40;
        replay_capture_site = sgBase + 0x9E8F0;
        f5_done_site = sgBase + 0x14199F;
        stop_site = sgBase + 0x1408F0;
        player_base = sgBase + ROOT_PTR_OFFSET;
        level_path_ptr = sgBase + GameAddresses::LEVEL_PATH_PTR_OFFSET;

        key_down_site = hmgBase + 0x3940;
        key_up_site = hmgBase + 0x3980;
        bb3b10 = hmgBase + 0x3B10;

        // Verify every required hook site before installing any hook; this also
        // catches a locally modified v1.035 image. Optional features validate
        // their own sites. All bytes are the on-disk form.
        static constexpr uint8_t kTickCave[] =                       // cmp esi,0x14; mov [esp+0x3C],esi
            { 0x83, 0xFE, 0x14, 0x89, 0x74, 0x24, 0x3C };
        static constexpr uint8_t kCycleCave[] =                       // push ebp; mov ebp,esp; push -1; push (SEH)
            { 0x55, 0x8B, 0xEC, 0x6A, 0xFF, 0x68 };
        static constexpr uint8_t kReplay[] =                      // sub esp,0x80
            { 0x81, 0xEC, 0x80, 0x00, 0x00, 0x00 };
        static constexpr uint8_t kKeyDown[] =                     // sub esp,8; push esi; mov esi,ecx
            { 0x83, 0xEC, 0x08, 0x56, 0x8B, 0xF1 };
        static constexpr uint8_t kKeyUp[] =
            { 0x83, 0xEC, 0x08, 0x56, 0x8B, 0xF1 };
        static constexpr uint8_t kBb3b10[] =                      // push -1; push HMG+0x583A (relocated)
            { 0x6A, 0xFF, 0x68, 0x3A, 0x58, 0x00, 0x10 };
        // Player::Player (SG+0x83DF0) at +0x5F: mov [esi+1C4],ebx;
        // mov dword ptr [esi], offset Player vtable (relocated imm32 at +8).
        static constexpr uint8_t kPlayerCtor[] =
            { 0x89, 0x9E, 0xC4, 0x01, 0x00, 0x00, 0xC7, 0x06, 0x10, 0x9E, 0x16, 0x10 };
        // Ghost_Player::Ghost_Player (SG+0x7FF10) at +0x28:
        // mov dword ptr [ebp+0], offset Ghost_Player vtable (imm32 at +3).
        static constexpr uint8_t kGhostCtor[] =
            { 0xC7, 0x45, 0x00, 0x74, 0x9B, 0x16, 0x10 };
        // The race loop's F5 poll: mov eax,[ecx]; mov edx,0x58 (F5);
        // call [eax+0x14] (Win32_Keyboard::State); test al,al; je +0x29.
        static constexpr uint8_t kF5Poll[] =
            { 0x8B, 0x01, 0xBA, 0x58, 0x00, 0x00, 0x00, 0xFF, 0x50, 0x14, 0x84, 0xC0, 0x74, 0x29 };
        // Its accepted branch: mov edx,[ebp]; mov ecx,ebp; call [edx] (restart).
        static constexpr uint8_t kF5Accept[] = { 0x8B, 0x55, 0x00, 0x8B, 0xCD, 0xFF, 0x12 };
        // Set_Game_Mode, after the mode-init call: call SG+0x13E410; mov eax,[eax+0x17C].
        static constexpr uint8_t kF5Done[] =
            { 0xE8, 0x6C, 0xCA, 0xFF, 0xFF, 0x8B, 0x80, 0x7C, 0x01, 0x00, 0x00 };
        // Race loop, after Set_Game_Mode: lea ecx,[esp+0xB4]; mov [esp+0x60],ebx; mov [esp+0x64],ebx.
        static constexpr uint8_t kLaunch[] =
            { 0x8D, 0x8C, 0x24, 0xB4, 0x00, 0x00, 0x00, 0x89, 0x5C, 0x24, 0x60, 0x89, 0x5C, 0x24, 0x64 };
        // Message pump entry: sub esp,0x24; push edi; push 0 (x4).
        static constexpr uint8_t kPump[] =
            { 0x83, 0xEC, 0x24, 0x57, 0x6A, 0x00, 0x6A, 0x00, 0x6A, 0x00, 0x6A, 0x00 };
        // Supreme::Stop entry: push esi; mov esi,ecx; call rel32; call rel32; mov ecx,[eax+0x40].
        static constexpr uint8_t kStop[] =
            { 0x56, 0x8B, 0xF1, 0xE8, 0xA8, 0x6C, 0xFC, 0xFF, 0xE8, 0x13, 0xDB, 0xFF, 0xFF, 0x8B, 0x48, 0x40 };
        if (!ValidateCode("Supreme.exe+0x25C81", tick_cave_site, kTickCave) ||
            !ValidateCode("Supreme.exe+0x25BD7 (race launched)", launch_site, kLaunch) ||
            !ValidateCode("Supreme.exe+0x55920 (message pump)", pump_site, kPump) ||
            !ValidateCode("Supreme_Game.dll+0x1408F0 (Supreme::Stop)", stop_site, kStop) ||
            !ValidateCode("Supreme.exe+0x25C0F (F5 poll)", exeBase + 0x25C0F, kF5Poll) ||
            !ValidateCode("Supreme.exe+0x25C3F (F5 accept)", f5_accept_site, kF5Accept) ||
            !ValidateCode("Supreme_Game.dll+0x14199F (Set_Game_Mode done)", f5_done_site, kF5Done) ||
            !ValidateCode("Supreme_Game.dll+0x13FE40", cycle_cave_site, kCycleCave) ||
            !ValidateCode("Supreme_Game.dll+0x9E8F0", replay_capture_site, kReplay) ||
            !ValidateCodeOrHooked("HMG_Cetsup_Win32.dll+0x3940", key_down_site, kKeyDown) ||
            !ValidateCodeOrHooked("HMG_Cetsup_Win32.dll+0x3980", key_up_site, kKeyUp) ||
            !ValidateCodeAbs<3>("HMG_Cetsup_Win32.dll+0x3B10", bb3b10, kBb3b10, hmgBase, 0x583A) ||
            !ValidateCodeAbs<8>("Supreme_Game.dll+0x83E4F (Player ctor)", sgBase + 0x83E4F, kPlayerCtor,
                                sgBase, PLAYER_VTABLE_RVA) ||
            !ValidateCodeAbs<3>("Supreme_Game.dll+0x7FF38 (Ghost_Player ctor)", sgBase + 0x7FF38,
                                kGhostCtor, sgBase, GHOST_PLAYER_VTABLE_RVA)) {
            return false;
        }
        player_vtable = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(sgBase + PLAYER_VTABLE_RVA));
        ghost_vtable = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(sgBase + GHOST_PLAYER_VTABLE_RVA));
        Log(std::format("Rider class vtables: Player {:#010x}, Ghost_Player {:#010x}", player_vtable, ghost_vtable));

        Log(std::format("EXE base: {:p}", (void*)exeBase));
        Log(std::format("SG base: {:p}", (void*)sgBase));
        Log(std::format("HMG base: {:p}", (void*)hmgBase));
        Log(std::format("the cycle cave site (Supreme::Cycle): {:p} (SG+0x13FE40)", (void*)cycle_cave_site));
        Log(std::format("the tick cave site (tick override): {:p} (EXE+0x25C81)", (void*)tick_cave_site));
        Log(std::format("Replay capture site: {:p} (SG+0x9E8F0)", (void*)replay_capture_site));
        Log(std::format("the key-handler cave down (+3940): {:p}", (void*)key_down_site));
        Log(std::format("the key-handler cave up (+3980): {:p}", (void*)key_up_site));
        Log(std::format("BB3B10 (+3B10): {:p}", (void*)bb3b10));
        Log(std::format("Player base ptr: {:p}", (void*)player_base));
        Log(std::format("Kernel::Time::Current: {:p}", (void*)time_current));

        return true;
    }
};
