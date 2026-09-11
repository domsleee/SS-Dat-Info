# F7 replay behaviour and live checks

F7 replays an available ghost: a loaded guide/TOP5 rider or a completed human
run. With no ghost loaded, it logs a refusal and leaves the current run alone.
Starting a replay hides both the results table and the replay instructions.

The helper reads the captured Player_Handler's ghost list and requires its
level root to match the current root before reading it. It clears the exe's
Game results flag and uses MSVCP60's string assignment to blank the prompt.
No synthetic SPACE key is needed.

## Results object identities

The results flag is shared by the Arcade Game subclasses, but each has a
different vtable. All four identities are accepted; unknown objects are not.

| Mode | Supreme.exe vtable RVA |
| --- | --- |
| Time Attack | `0x6D6EC` |
| Race | `0x6D730` |
| Pipe | `0x6D6A8` |
| Air | `0x6D664` |

These are assigned in Supreme.exe's Arcade sequence constructor at `+0x30E0`,
after the common base constructor at `+0x183C0`. All four addresses were also
read from actual game objects during the 2026-09-11 validation.

## Regression reproduced and fixed

On branch head `a0568cf`, a natural Forest Pipe finish displayed its results.
Pressing F7 restarted the replay and blanked the prompt, but left the table
visible: the results flag remained 1 and the helper logged
`F7: exe Game object not found - results page left as is`.

The corrected build accepts Pipe's Game subclass. Repeating that finish and
F7 sequence changed the flag from 1 to 0, cleared the prompt string, and
removed both overlays on screen. A natural Forest Air finish passed the same
check. The test did not write either flag or prompt from outside the helper.

Further live checks on that build:

- Empty Pipe and Air runs: three F7 presses each were refused without a dialog.
- Finished Pipe replay: ten repeated F7 presses left both overlays hidden.
- SPACE showed the Pipe results again; F7 hid them again.
- F5 after the Pipe replay removed the finished-run ghost; three F7 presses
  were then refused without a dialog.
- Finished Air replay: five repeated F7 presses remained stable.
- Forest Easy guide enabled: F7 played the guide with no prompt; ten repeated
  presses and F7 after F5 remained usable.
- Return through the menu, disable the guide, and re-enter Forest Easy:
  five F7 presses were refused rather than using the old level's ghost.
- Finished Forest Easy Time Attack using the FE-decent-done input recording:
  after declining the save prompt, F7 cleared both overlays. Five further
  presses remained stable; F5 then three F7 presses safely refused the now
  empty replay.

Race's object identity was measured live, but a Time Attack input fixture did
not finish the Race course. The full Race finish-to-F7 sequence was not
established by that trial; Pipe and Air provided the before/after coverage of
the shared results-flag correction.

## Build and review

The Win32 Release helper build passed. A fresh gpt-6-astra medium review found
the Time-Attack-only vtable check; a follow-up review of the correction returned
no findings. The tested DLL SHA-256 was
`3389AF0A10F59E2C810F9FA3E227CFD518145BE64D7CD0EAF03BCA75AF4467E1`.

Screenshots, helper-log excerpts, read-only object-state snapshots, and both
review outputs are saved locally under the TAS worktree's
`TAS/artifacts/f7-review-20260911/`. The helper was staged into that worktree's
isolated game copy for testing.

## Manual review

1. Start Time Attack with no guide or TOP5 ghost and press F7: gameplay should
   continue and the helper log should say there is nothing to replay.
2. Enable a guide, enter the track, and press F7: its replay should start.
3. Finish a run in each Arcade mode, dismiss any save prompt, and press F7:
   the replay should start with neither the table nor the press-space line.
4. Press F7 again, then SPACE and F7: the table should end hidden each time.
5. F5 into a new run without a loaded ghost, and re-enter through the menu:
   F7 should refuse the empty replay without crashing.
