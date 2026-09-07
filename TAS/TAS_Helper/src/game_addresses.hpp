#pragma once
#include "stdafx.h"
#include "log.hpp"

// Game module bases and resolved addresses. Every offset is a fixed RVA of the
// stock v1.035 images; ValidateModule / ValidateCode* in Resolve are the pin.

// Housemarque Kernel::Time — a 64-bit timestamp ({lo, hi} dwords; ~QPC-derived
// machine-uptime units). The input pipeline stamps every key event with the
// Time at message-pump dispatch: Win32_Driver::Translate(tagMSG&, Kernel::Time)
// → keyDown/keyUp (+3940/+3980) → BB3B10(keyIndex, pressed, Time.lo, Time.hi).
// The observer discards events whose Time predates the current race context —
// the "dynamic arg4" that silently killed stale injected input.
struct KernelTime { uint32_t lo; uint32_t hi; };

// ?Current@Time@Kernel@Housemarque@@SI?AV123@XZ — static __fastcall, returns
// the Time by value through a hidden return slot passed in ecx (verified by
// disasm: `mov esi, ecx; ... mov [esi], eax; mov [esi+4], edx; mov eax, esi`).
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
    // HMG_Cetsup_Win32 and SR_UIT all prefer ImageBase 0x10000000 and are
    // always relocated, so any imm32 the loader fixes up differs from the
    // on-disk bytes. Use ValidateCodeAbs for those.
    template <size_t N>
    static bool ValidateCode(const char* label, const std::uint8_t* address,
                             const std::uint8_t (&expected)[N]) {
        return ValidateCodeBytes(label, address, expected, N);
    }

    // Compare live code whose imm32 at `AbsOffset` is an absolute address the
    // loader rebases. `expected` holds the on-disk bytes; the immediate is
    // rewritten to `moduleBase + absRva` before comparing, so the check holds
    // wherever the module landed. (Verified against the .reloc tables: each of
    // these sites carries a HIGHLOW fixup at +3.)
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

    // Like ValidateCode, but also accepts a relative JMP (E9) at the site.
    // Display_Config_Helper is injected BEFORE this DLL (handlePlay.ts) and
    // inline-hooks the two HMG key handlers for its F5 debounce, so on a normal
    // launch those sites already start with a JMP. The module identity check
    // still proves which image this is, and SafetyHook chains onto the
    // existing hook (it did on every launch before byte validation existed).
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

    // Module bases
    HMODULE exe = nullptr;   // Supreme.exe
    HMODULE sg = nullptr;    // Supreme_Game.dll
    HMODULE hmg = nullptr;   // HMG_Cetsup_Win32.dll
    HMODULE kernel = nullptr; // HMG_Kernel.dll

    // HMG_Kernel.dll: Kernel::Time::Current() — the game's own clock. Used to
    // stamp injected BB3B10 input events with a genuinely-current timestamp
    // (exactly what a real keypress carries), so injection is never discarded
    // as stale. nullptr = export missing (fall back to calibrated arg4).
    KernelTimeCurrentFn time_current = nullptr;

    // Supreme.exe offsets
    std::uint8_t* cave5_site = nullptr;     // exe+0x25C81: after ftol+mov esi,eax (tick override)
    std::uint8_t* is_in_game = nullptr;     // exe+0x8895C: 0=main menu, 1=in-game (race/level)

    // Supreme_Game.dll offsets
    std::uint8_t* cave2_site = nullptr;     // SG+0x13FE40: Supreme::Cycle
    std::uint8_t* replay_capture_site = nullptr; // SG+0x9E8F0: replay object capture
    std::uint8_t* player_base = nullptr;    // SG+0x1D5450: root pointer
    // Live vtable addresses (SG base + RVA) the replay-capture hook classifies
    // recorder owners with; 0 until Resolve validated the constructor sites.
    std::uint32_t player_vtable = 0;
    std::uint32_t ghost_vtable = 0;
    // SG+0x1D3304: pointer to the CURRENT level's resource path string.
    // Found by differential RE (tas_test level-hunt ptr) and verified on four
    // tracks. The pointed-to string changes the instant a level loads, which is
    // the level-change event. RELIABLE FOR AREA, NOT FOR DIFFICULTY: some tracks
    // share the easy/ shadow asset, so Village Hard reads ".../Tracks/easy/...".
    // Difficulty comes from the executable's selected-track config object;
    // see setup_config_parse.hpp. That survives the shared-path outlier.
    std::uint8_t* level_path_ptr = nullptr;
    std::uint8_t* vk_table = nullptr;       // SG+0x9AD8: VK -> keyIndex lookup

    // HMG_Cetsup_Win32.dll offsets
    std::uint8_t* cave1c_down = nullptr;    // HMG+0x3940: key handler (down)
    std::uint8_t* cave1c_up = nullptr;      // HMG+0x3980: key handler (up)
    std::uint8_t* bb3b10 = nullptr;         // HMG+0x3B10: BB3B10 observer

    // Game key codes: the DI-buffer index and the keyIndex BB3B10 broadcasts
    // to the observers are the same value.
    static constexpr uint32_t KEY_UP     = 0x38;
    static constexpr uint32_t KEY_DOWN   = 0x39;
    static constexpr uint32_t KEY_LEFT   = 0x3A;
    static constexpr uint32_t KEY_RIGHT  = 0x3B;
    static constexpr uint32_t KEY_JUMP   = 0x27;  // LCTRL
    static constexpr uint32_t KEY_JUMP2  = 0x28;  // CTRL (duplicate)
    static constexpr uint32_t KEY_SHIFT  = 0x24;  // SHIFT
    static constexpr uint32_t KEY_SHIFT2 = 0x25;  // LSHIFT (duplicate)
    static constexpr uint32_t KEY_F5     = 0x58;  // restart race
    // ESC (VK 0x1B). The pause menu listens for it through the BB3B10 observer
    // broadcast, so the REC-mode observer block must exempt it.
    static constexpr uint32_t KEY_ESC    = 0x48;

    // Pointer chain: root = [SG+1D5450], kbobj = [root+530], buffer = [kbobj+30]
    static constexpr uint32_t ROOT_PTR_OFFSET = 0x1D5450;
    static constexpr uint32_t LEVEL_PATH_PTR_OFFSET = 0x1D3304;
    static constexpr uint32_t KEYBOARD_OBJ_OFFSET = 0x530;
    static constexpr uint32_t DI_BUFFER_PTR_OFFSET = 0x30;

    // Action state byte offsets from keyboard object (kbobj = [root+0x530])
    static constexpr uint32_t AS_LEFT  = 0x455;
    static constexpr uint32_t AS_UP    = 0x456;
    static constexpr uint32_t AS_RIGHT = 0x457;
    static constexpr uint32_t AS_DOWN  = 0x458;
    static constexpr uint32_t AS_SHIFT = 0x440;
    static constexpr uint32_t AS_JUMP  = 0x441;

    // Replay object: player ptr at [replayObj+0x84]
    static constexpr uint32_t REPLAY_PLAYER_OFFSET = 0x84;
    // Human-player identity (RTTI): the rider the keyboard drives in a race is
    // a plain `Player` (the base class, vtable below); Time Attack ghosts / the
    // guide are `Ghost_Player`, computer riders `AI_Player`, network riders
    // `Net_Player` - each with its own vtable. The player links back to its
    // recorder at +0x14C (decompile param_1[0x53]); the replay-capture hook
    // adopts a recorder only if its owner is a Player that still points at it
    // (see replay_identity.hpp). Both vtable RVAs are validated against the
    // constructors' `mov [this], offset vtable` immediates in Resolve.
    static constexpr uint32_t PLAYER_RECORDER_OFFSET = 0x14C;
    // Rider identity (live object-graph dump). The Player's
    // loadout object ([player+0x20], no RTTI) holds MSVC6 std::strings
    // ({allocator, char* ptr, size, capacity} = 16 bytes each): the character
    // folder at +0x10 ("vincent"), the character config path at +0x30
    // ("data/characters/vincent/plrcnf.txt"), the board config path at +0x60
    // ("data/boards/board_1/boardcnf.txt" - the board does not affect the
    // physics, not published), plus a word at +0x20 whose low half flips
    // with the stance (published raw). The Player_Config ([player+0x48],
    // RTTI Player_Config) carries the display name as a std::string at +0x48
    // ("Vincent").
    static constexpr uint32_t PLAYER_LOADOUT_OFFSET = 0x20;
    static constexpr uint32_t PLAYER_CONFIG_OFFSET = 0x48;
    static constexpr uint32_t LOADOUT_FOLDER_STRING = 0x10;
    static constexpr uint32_t PLAYER_CONFIG_NAME_STRING = 0x48;
    static constexpr uint32_t MSVC6_STRING_PTR = 0x4;
    static constexpr uint32_t MSVC6_STRING_SIZE = 0x8;
    // Stance and the authoritative selected area/difficulty live in the game
    // config object read by setup_object.hpp. Its independently testable chain
    // and field offsets are kept in setup_config_parse.hpp.
    static constexpr uint32_t PLAYER_VTABLE_RVA = 0x169E10;        // .?AVPlayer@Supreme_Snowboarding@Housemarque@@
    static constexpr uint32_t GHOST_PLAYER_VTABLE_RVA = 0x169B74;  // .?AVGhost_Player@...
    // Player position offsets
    static constexpr uint32_t PLAYER_X = 0xF8;
    static constexpr uint32_t PLAYER_Y = 0xFC;
    static constexpr uint32_t PLAYER_Z = 0x100;
    static constexpr uint32_t PLAYER_PHYSICS = 0x110; // Pointer to physics sub-object
    static constexpr uint32_t PHYSICS_ROT = 0x1B4;   // 3x3 rotation matrix in physics sub-object (9 floats, row-major)

    // Fallback Time.hi for injected BB3B10 calls when Kernel::Time::Current is
    // unavailable: what the real key handler passes for a steering key (live
    // capture). A stale stamp is discarded by the observer, which turns the
    // injected call into a silent no-op.
    static constexpr uint32_t BB3B10_ARG4 = 0x96;
    // BB3B10 this pointer offset from keyboard object
    static constexpr uint32_t BB3B10_THIS_OFFSET = 0x18;

    bool Resolve() {
        exe = GetModuleHandleA(nullptr);  // Supreme.exe (main executable)
        sg = GetModuleHandleA("Supreme_Game.dll");
        hmg = GetModuleHandleA("HMG_Cetsup_Win32.dll");
        kernel = GetModuleHandleA("HMG_Kernel.dll");

        if (!exe) { Log("ERROR: Supreme.exe not found"); return false; }
        if (!sg) { Log("ERROR: Supreme_Game.dll not loaded"); return false; }
        if (!hmg) { Log("ERROR: HMG_Cetsup_Win32.dll not loaded"); return false; }

        // Every address below is a fixed RVA for the stock v1.035 image. Refuse
        // to patch a merely-similar process: a different executable or DLL build
        // can put unrelated instructions at the same offsets.
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

        // Kernel::Time::Current — non-fatal if missing (injection falls back
        // to the keypress-calibrated arg4), but it should always resolve.
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

        // Supreme.exe offsets
        cave5_site = exeBase + 0x25C81;
        is_in_game = exeBase + 0x8895C;  // 0=menu, 1=in-game (RE'd via menu↔game diff)

        // Supreme_Game.dll offsets
        cave2_site = sgBase + 0x13FE40;
        replay_capture_site = sgBase + 0x9E8F0;
        player_base = sgBase + ROOT_PTR_OFFSET;
        level_path_ptr = sgBase + GameAddresses::LEVEL_PATH_PTR_OFFSET;
        vk_table = sgBase + 0x9AD8;

        // HMG_Cetsup_Win32.dll offsets
        cave1c_down = hmgBase + 0x3940;
        cave1c_up = hmgBase + 0x3980;
        bb3b10 = hmgBase + 0x3B10;

        // Verify every REQUIRED code target before installing ANY hook. Module
        // metadata catches different releases; these signatures also catch a
        // locally-modified image of the supported release. Optional features
        // (race timer) validate their own sites and degrade instead of failing
        // initialization. All bytes are the on-disk form.
        static constexpr uint8_t kCave5[] =                       // cmp esi,0x14; mov [esp+0x3C],esi
            { 0x83, 0xFE, 0x14, 0x89, 0x74, 0x24, 0x3C };
        static constexpr uint8_t kCave2[] =                       // push ebp; mov ebp,esp; push -1; push (SEH)
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
        if (!ValidateCode("Supreme.exe+0x25C81", cave5_site, kCave5) ||
            !ValidateCode("Supreme_Game.dll+0x13FE40", cave2_site, kCave2) ||
            !ValidateCode("Supreme_Game.dll+0x9E8F0", replay_capture_site, kReplay) ||
            !ValidateCodeOrHooked("HMG_Cetsup_Win32.dll+0x3940", cave1c_down, kKeyDown) ||
            !ValidateCodeOrHooked("HMG_Cetsup_Win32.dll+0x3980", cave1c_up, kKeyUp) ||
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
        Log(std::format("Cave 2 site (Supreme::Cycle): {:p} (SG+0x13FE40)", (void*)cave2_site));
        Log(std::format("Cave 5 site (tick override): {:p} (EXE+0x25C81)", (void*)cave5_site));
        Log(std::format("Replay capture site: {:p} (SG+0x9E8F0)", (void*)replay_capture_site));
        Log(std::format("Cave 1C down (+3940): {:p}", (void*)cave1c_down));
        Log(std::format("Cave 1C up (+3980): {:p}", (void*)cave1c_up));
        Log(std::format("BB3B10 (+3B10): {:p}", (void*)bb3b10));
        Log(std::format("Player base ptr: {:p}", (void*)player_base));
        Log(std::format("VK table: {:p}", (void*)vk_table));
        Log(std::format("Kernel::Time::Current: {:p}", (void*)time_current));

        return true;
    }
};
