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
}

export interface TimingDataFromHeader {
  totalRecordingTimeMs: number;
  crossStartPlusStartDelayMs: number;
  checkpoint1TotalMs: number;
  totalTimeToFinishMs: number;
}
