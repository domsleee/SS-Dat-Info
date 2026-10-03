# M0 provenance and launch repairs

Branch `codex/m0-final-review`, based on `bf52b97`. No deployment or live
game/TAS interaction was performed.

The fixed trace header can no longer omit a new file identity observed during
PLAY and still publish successfully. `trace_provenance.hpp` keeps the existing
file/hash set and a generation counter under one mutex. The header snapshots
that generation with its file lines. Close checks the generation and holds the
same mutex through checked flush, close and rename. A changed set adds status
32, so the existing publication helper rejects the trace and removes pending
output where possible. Identical reopens do not change the generation.

An observation serialized after publication belongs to the next snapshot:
the intercepted open cannot return to game code until its observation finishes.
This is deliberately conservative. A legitimate lazy asset load during PLAY
invalidates the take; it must be resolved before claiming complete provenance.
No extra artifact format or permissive identity exclusion was introduced.

`Sha256File` now distinguishes failed reads from successful EOF. It returns
`unreadable` on failure, never a valid digest of the bytes read before the
failure. A failed identity before the header makes the header invalid to the
checker; one first observed after the snapshot also prevents publication.

`tas_test` launches Supreme directly with `Command`, sets its working directory,
disconnects standard streams, reports the child PID, and waits only for that
child's game window. This preserves direct ancestry for the outer runner without
letting the game retain redirected log pipes. The fresh-launch path refuses a
remaining game window or shared mapping instead of calling image-name-based
`kill_game`. Explicit destructive test modes still have their existing reset
operations; the ordinary replay path does not call them. Existing live-session
reuse behavior is unchanged, so the outer M0 runner's idle/ownership gate remains
necessary. Launch/injection against a real game was not tested here.

Validation:

- `just test_dll`: all 17 native suites pass, x86, hidden child windows.
- New native controls cover the known SHA-256 of `abc`, first and late read
  failures, unchanged/new/changed/unreadable reopens, 50 snapshot races, and an
  observation blocked through publication. Failed provenance never publishes
  the otherwise valid zero-status provisional footer.
- `just tas_dll`: Release Win32 helper builds.
- `cargo test -p tas_test --release`: 77 tests pass.
- `cargo build -p tas_test --release`, `cargo fmt --all --check`, and
  `cargo clippy -p tas_test --all-targets -- -D warnings`: pass.

Artifacts in this worktree:

- `TAS/TAS_Helper/Release/TAS_Helper.dll`
- `TAS/target/release/tas_test.exe`

Ponytail review: reused checked publication and standard mutex/process APIs;
removed the now-unused ShellExecute wrapper. No dependency, alternate session
namespace, capture codec or generic process framework was added. These repairs
close recorder/runner defects; they do not supply fresh M0 corpus evidence.
