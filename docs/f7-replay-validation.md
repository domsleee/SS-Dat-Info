# F7 validation

F7 should replay a loaded guide/TOP5 ghost or finished run, with the results
table and press-space prompt hidden. With nothing to replay, it should leave
gameplay alone and log a refusal.

Live checks on an isolated game copy, 2026-09-11:

| Scenario | Result |
| --- | --- |
| No ghost, including after F5 and menu re-entry | F7 refused; no crash or error dialog |
| Guide loaded, including after F5 | Guide replayed; prompt hidden |
| Finished Time Attack, Pipe and Air runs | F7 replayed; table and prompt hidden |
| Repeated F7; SPACE followed by F7 | Replay remained usable; results ended hidden |

The Pipe test caught a results-table check that only recognised Time Attack.
The fix accepts the four Arcade game classes; the same finish-to-F7 test then
passed in Pipe and Air. Race's object identity was verified live, but its full
finish-to-F7 sequence remains untested.

Win32 Release build passed. Astra reviewed the fix with no remaining findings.
