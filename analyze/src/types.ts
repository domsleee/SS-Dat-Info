export interface AnalyzeResult {
  playerName: string;
  trackName: string;

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
  rx: number;
  rw: number;
  ry: number;
  rz:number;
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

export const REPLAY_FOLDER = "../replays";

