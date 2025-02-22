import { fileURLToPath } from "bun";
import { dirname, resolve } from "path";

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
  // checkpoint2Ms: number;

  // Speed
  // routeLength: number;
  // averageSpeed: number;
  // maximumSpeed: number;

  // distanceBreaksBeforeFinish: number;
  // distanceBreaksAfterFinish: number;
  // speedBreaksBeforeFinish: number;
  // speedBreaksAfterFinish: number;

  coords?: CoordinateData;
}

export interface CoordinateData {
  rows: Array<RowData>;
}

export interface RowData {
  x: number;
  y: number;
  z: number;
  rotation3x3: Array<Array<number>>;
  raw: string;
  movementState: MovementState;
  left: boolean;
  right: boolean;
  forward: boolean;
  shift: boolean;
}


export enum MovementState {
  NOPRESS = "NOPRESS",
  FORWARD = "FORWARD",
  LEFT = "LEFT",
  RIGHT = "RIGHT",
  FORLEFT = "FORLEFT",
  FORRIGHT = "FORRIGHT",
}

export interface TimingDataFromHeader {
  totalRecordingTimeMs: number;
  crossStartPlusStartDelayMs: number;
  checkpoint1TotalMs: number;
  totalTimeToFinishMs: number;
}

export const REPLAY_FOLDER = resolve(import.meta.dir, "../../threejs/public/replays");
