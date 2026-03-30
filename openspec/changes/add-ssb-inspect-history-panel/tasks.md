## 1. History Model

- [ ] 1.1 Replace the undo-only `UndoRing` with a bounded recording history type that supports snapshot entries, marker entries, a current cursor, and branch truncation after new edits
- [ ] 1.2 Update save/load helpers to append successful `Save to file` and `Load from file` history entries with file context while keeping failed or cancelled operations out of history
- [ ] 1.3 Add `recording.rs` unit tests for history push/undo/redo/select behavior, save markers, load-created states, and capacity eviction

## 2. SSB Inspect UI

- [ ] 2.1 Add a new `History` panel module and render it as an optional side panel with an empty state, ordered entries, current-entry highlight, and click-to-restore behavior
- [ ] 2.2 Add persisted `show_history` settings and expose the panel through the existing `View` menu
- [ ] 2.3 Route transport actions, file-menu actions, and direct history selection through shared history helpers so undo, redo, save, and load stay synchronized with the visible history list
- [ ] 2.4 Merge the current and pending redo behavior into the shared history model, including transport affordances and keyboard shortcut handling

## 3. Verification

- [ ] 3.1 Extend `main.rs` and related module tests for redo shortcut routing, history panel visibility defaults, and persisted settings behavior
- [ ] 3.2 Run `cargo test -p tas_ui` and fix any regressions introduced by the history feature
- [ ] 3.3 Perform a manual `SSB Inspect` smoke test covering undo, redo, save, load, and direct history selection from the new panel
