#pragma once

namespace GlobalState {
  static bool ghostsOpaque = false;
  // A run has finished in the current level: the game wrote its replay
  // (saveReplayTimestamp.hpp sees the write), so Set_Replay_Mode has
  // something to play. Cleared when the level root changes (customInput.hpp).
  static bool replayReady = false;
  // The finish results overlay (hiscore table) is on screen. Raised with
  // replayReady; customInput.hpp tracks the SPACE toggles that flip it.
  static bool resultsVisible = false;
}
