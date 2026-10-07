# Game calls from hook bodies run at the game's FPU control state

2026-10-07. Branch `claude/fpu-cw` (from `claude/m0-census`). Offline only.

## Evidence

ssb-recomp `re/m1-fpu-env.md` (branch `claude/m1-fpuenv`): with the HTL table
detoured, ssb-hook saw HTL entries at x87 CW 0x037F (64-bit precision) instead
of the game's 0x027F (53-bit). Counts per launch were deterministic: 400
FE-tremendous, 340 AH-best-5307, 252 VH-best-4549, all before the first race.
The game never selects PC64 (capstone sweep of all 44 binaries: no
`finit`/`frstor`/`fldenv`/`fxrstor`, every `fldcw` pair selects PC24). The
cause is TAS_Helper: `FpuSafeCall` runs every mid-hook body after `fsave`,
which re-initialises the FPU to CW 0x037F, and the menu channel
(`ExecuteCb` at Main_Menu.dll+0x1A680) calls the game's menu code from that
body. Main_Menu.dll and UIT.dll import 11 and 4 of the HMG_HTL_2 maths
functions. At PC64, V3_V3_Sub_V3_Dot differs from PC53 in 996 of 1000
random inputs.

## Fix

`fpu_safe_hook.hpp` adds `GameFpuScope`. It saves the body's control word
and MXCSR, loads the values `FpuSafeCall` saved at hook entry
(`g_hookFpuControlWord` from the FSAVE image, `g_hookMxcsr` from `stmxcsr`),
and puts the body's values back at scope exit. Nothing is hard-coded: a game
running at PC24 (DirectX renderers) gets PC24.

`crash::ScopedGameCall`, which already marks every DLL call into game code,
now holds a `GameFpuScope`. The one game call without it, the BB3B10 key
injection loop, gets its own scope. Hook bodies that call no game code are
unchanged. Every call-out from a hook body:

| Site | Game code | Hook body |
| --- | --- | --- |
| `menu_cave.hpp` `ReadItem` | `Want_Focus` (vtable slot 0x2C) | Execute (snapshot and command) |
| `menu_cave.hpp` `ActiveComponent` | `UI_Menu::Get_Active_Component` (MM+0x1A6B0), 2 callers | Execute |
| `menu_cave.hpp` `ReadMenu` | `UI_Container::Get_Modal` (UIT export) | Execute |
| `menu_cave.hpp` `RunCommand` | `UI_Menu::Up/Down/Left/Right/Trigger`, `UI_Component::Request_Focus` (UIT+0x196D0) | Execute |
| `cycle_cave.hpp` `FlushObserverQueue` | TC_Kbd_Impl flush | Cycle, Pump, Stop |
| `cycle_cave.hpp` `CallBB3B10OnTransitions` | `Kernel::Time::Current`, BB3B10 | Cycle |
| `cycle_cave.hpp` `TryProcessTestFault` | `Kernel::Time::Current` (tas_test crash test) | Pump |

The other hook bodies (tick, race timer, finish, F5, replay capture, trace
hooks) only read and write memory. The inline detours (key handler, observer)
do not FSAVE, so the game's own state passes through them.

`g_hookFpuControlWord` and `g_hookMxcsr` are now per thread and restored when
a body returns. Before, a nested hook (for example Change_Page under
Trigger, or a hook inside one of the game's PC24 sections) left its own
entry values behind for the outer body's next game call.

### Why `fldcw` and `ldmxcsr`, not `fldenv`

Only the control word (precision, rounding, masks) and the MXCSR control bits
change results. The x87 stack the game call sees is already what the game has
at any call: FSAVE leaves the tags empty, and compiled code keeps the stack
empty across calls. `fldenv` of the saved image would also bring back the
game's tag word (which then has to be forced empty), its status word and any
pending-exception bits, and it needs an `fnstenv`/`fldenv` pair to restore the
helper. That is more code for no difference in results.

What stays different from a real key press: sticky exception flags. The game
call starts with a clear x87 status word, and flags it raises are dropped at
hook exit by `frstor` and `ldmxcsr`, as before this change. All exceptions
are masked and no game binary imports `_statusfp`, `_clearfp`, `_status87`
or `_clear87`, so I expect no effect, but I have not checked for inline
`fnstsw`/`stmxcsr` reads of the sticky bits.

## Tests

`TAS/TAS_Helper/src/tests/test_fpu_game_call.cpp`: enters `FpuSafeCall` at
CW 0x027F/MXCSR 0x1FA0, and again at CW 0x047F/MXCSR 0xBF80, with values on
the x87 stack. The body opens two `GameFpuScope`s around a probe standing in
for game code. The probe records `fnstcw`, `stmxcsr` and the tag word, then
enters a nested hook at PC24. Both probe calls must see the entry CW and MXCSR
and an empty stack (tags 0xFFFF). The body must run at 0x037F before and
after, with its MXCSR unchanged. The globals must be 0 after the outermost
hook. Negative controls: without the scope, both CW checks fail; without the
nested restore, the second call's check fails.

## Live confirmation (coordinator)

Rebuild the DLL from this branch and rerun step 1 of `re/m1-fpu-env.md`
("Live commands") with `$H` pointing at it. For FE-tremendous,
AH-best-5307 and VH-best-4549:

- `ssb-trace info` prints `FPU violations 0 (off-PC53 entries 0)` on every
  trace, where it printed 400 / 340 / 252 with the old DLL.
- TAS_Helper.log has no `htl: ... not PC53 from Main_Menu.dll+.../UIT.dll+...`
  diagnostic lines, and no `off-PC53 entries so far` counts above 0.

If a nonzero count remains, the diagnostic line names the caller, which is a
call-out this audit missed.
