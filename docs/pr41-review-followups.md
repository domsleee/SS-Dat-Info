# PR 41 review follow-ups

Reviewed the OpenCode session `ses_f8e5d38f6ffe4PZQYOAwVerEMU` and its working-tree
changes on 2026-09-06. The chat accurately records the initial head-based warning
fix flickering during CONT and the subsequent change to a splice-only verdict.

## Additional defects found and corrected

- The splice-only verdict waited for sample N during PLAY. The DLL captures N-1,
  advances playback_pos, switches to REC and clears continue_from_frame. It never
  captures N in PLAY. The corrected scanner judges the last replayed pair and
  handles REC publication, including catch-up completing between UI polls.
- The fixture test did not call the production scanner/banner. The new main.rs
  regression feeds the captured samples through those functions across UI polls
  and asserts a completed verdict, not merely an absent warning. A separate
  divergent-boundary test failed against the reviewed code and passes with the fix.
- The original capture was not a zero-based prefix ending at 4500: its first
  16 samples identify recording tick 945. Documentation now identifies the actual
  coverage, and the capture script requires the requested segment, generation and
  gate-aligned endpoint before saving future fixtures.

The splice result describes X/Z coordinate agreement at the continuation boundary,
not full-state or full-prefix bit-exactness. Transient differences remain in the
debug plot and diagnostic log. Normal PLAY retains its historical drift policy.

## Earlier review findings

The working tree also contains the earlier fixes and regression tests for editor
detachment, stable/valid script saves, aligned debug graphs, launcher errors and
USB data-interface selection. Pico identification checks USB descriptors, not a
firmware handshake; the documented firmware remains required.

The DLL now acquires a separate single-owner handle before initializing shared
memory. Cross-process tests verify exclusion and recovery after owner exit, using
test-only names. All injected DLLs must include this guard; old loaded versions do
not participate. This build has not been injected into the user's running game.

## Still requiring live evidence

The reported restart crash remains unproven as fixed. Passing LEFT-spam retry and
held LEFT+SHIFT tests is not a crash reproduction. The 1024-tick CONT bucket check
also remains a bounded prefix check, not a full replay certificate. Neither result
should be represented as stronger than the evidence supports.
