## ADDED Requirements

### Requirement: Completed recording sessions create visible history entries
The system SHALL add a new visible history entry only when a `REC` or `CONT` session stops after increasing the recording length, and the entry SHALL represent the final recorded state of that completed session.

#### Scenario: Fresh recording creates a completed-session entry
- **WHEN** the user starts a fresh `REC` session, records new ticks, and then stops recording
- **THEN** the history list adds a new current entry for the completed recording session

#### Scenario: Continue recording creates a completed-session entry
- **WHEN** the user starts a `CONT` session from an existing recording, records additional ticks, and then stops recording
- **THEN** the history list adds a new current entry for the completed continue session

#### Scenario: Play-stop cycle does not create history noise
- **WHEN** the user plays back an existing recording and then stops without adding new recorded ticks
- **THEN** the history list does not add a new entry for that play-stop cycle

### Requirement: Session history labels use 100 Hz recording time
The system SHALL label completed recording-session entries with human-readable durations derived from the TAS tick rate of 100 ticks per second.

#### Scenario: Fresh recording label shows recorded duration
- **WHEN** a completed fresh recording session is added to history
- **THEN** its label uses the form `Recorded <duration>` where `<duration>` is formatted from recorded ticks at 100 Hz

#### Scenario: Continue label shows segment and total duration
- **WHEN** a completed continue session is added to history
- **THEN** its label uses the form `Continued <segment-duration>, total <total-duration>` with both durations formatted from ticks at 100 Hz

### Requirement: Crash recovery preserves the latest recoverable recording checkpoint
The system SHALL persist recoverable recording checkpoints outside the visible history list while recording progress is being made so a crash can be recovered without relying on a manual save.

#### Scenario: Active recording updates crash recovery data
- **WHEN** a `REC` or `CONT` session is in progress and the recorded tick count advances
- **THEN** the system updates a recoverable checkpoint for the latest persisted recording state without adding a visible history entry

#### Scenario: Crash recovery can restore the latest persisted recording
- **WHEN** `SSB Inspect` starts after an unclean shutdown and a recovery checkpoint exists
- **THEN** the app exposes a way to restore the latest persisted recording state and its matching session history context
