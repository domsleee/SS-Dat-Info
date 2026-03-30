## ADDED Requirements

### Requirement: History panel presents recording actions
The system SHALL provide an optional `History` panel in `SSB Inspect` that presents the current TAS editing session as a Photoshop-style vertical history stack with the active entry visibly highlighted.

#### Scenario: Empty history panel
- **WHEN** the `History` panel is visible before any history entries have been created
- **THEN** the panel shows an explicit empty-state message instead of stale or placeholder history items

#### Scenario: Session actions appear in order
- **WHEN** the user records, continues, undoes, redoes, saves, or loads during a session
- **THEN** the `History` panel shows those actions as ordered, user-readable entries

### Requirement: History navigation uses a single current-state cursor
The system SHALL back undo, redo, and direct history selection with a single bounded history model so the visible current entry and restored recording state remain synchronized.

#### Scenario: Undo moves to an earlier state
- **WHEN** the user triggers `Undo` and an earlier restorable history state exists
- **THEN** the system restores that earlier recording state and moves the highlighted history entry to that state

#### Scenario: Redo moves to a later state
- **WHEN** the user triggers `Redo` and a later restorable history state exists
- **THEN** the system restores that later recording state and moves the highlighted history entry to that state

#### Scenario: Selecting a prior history state
- **WHEN** the user selects a prior restorable entry in the `History` panel
- **THEN** the system restores the recording to that entry and marks it as the current state

### Requirement: File operations appear in history
The system SHALL show successful `Save to file` and `Load from file` actions in the `History` panel with enough file context for the user to distinguish them from edit actions.

#### Scenario: Save adds a visible marker
- **WHEN** the user successfully saves the current recording to disk
- **THEN** the `History` panel adds a save-labelled entry for that file without changing the current restored recording state

#### Scenario: Load becomes the current history state
- **WHEN** the user successfully loads a recording from disk
- **THEN** the `History` panel adds a load-labelled entry for that file and makes the loaded recording the current selected history state

### Requirement: History panel visibility is persisted
The system SHALL let users show or hide the `History` panel through the existing view controls and SHALL restore that preference on the next launch.

#### Scenario: User reopens the app
- **WHEN** the user enables or disables the `History` panel and later restarts `SSB Inspect`
- **THEN** the app restores the same `History` panel visibility state
