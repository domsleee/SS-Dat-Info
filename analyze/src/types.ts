import { EveryLevelScoredData } from "./PlaneUtil/types";

export interface AnalyzeResult {
  playerName: string;
  trackName: string | undefined;

  // timing
  displayedMs: number;
  startMs: number;
  totalMs: number;

  lagBeforeStartMs: number;
  lagAfterFinishMs: number;
  recordingMs: number;

  checkpoint1Ms: number;
  timingDataFromHeader: TimingDataFromHeader;

  coords?: CoordinateData;
  trackScoreData?: EveryLevelScoredData;
  data: Uint8Array;
}

export interface CoordinateData {
  rows: Array<RowData>;
}

export interface RowData {
  x: number;
  y: number;
  z: number;
  rotation3x3: Array<Array<number>>;
}

export interface TimingDataFromHeader {
  totalRecordingTimeMs: number;
  crossStartPlusStartDelayMs: number;
  checkpoint1TotalMs: number;
  totalTimeToFinishMs: number;
}

export interface AnalyzeReplayOptions {
  skipCoords: boolean;
}

export const UNKNOWN_TRACK = "Unknown Track";
