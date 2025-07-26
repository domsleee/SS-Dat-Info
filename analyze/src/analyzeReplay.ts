import { getEveryLevelScored } from "./PlaneUtil/scoreTrack";
import { AnalyzeReplayOptions, AnalyzeResult, CoordinateData, TimingDataFromHeader, UNKNOWN_TRACK } from "./types";

export function analyzeReplay(data: Uint8Array, options?: AnalyzeReplayOptions): AnalyzeResult {
  const { playerName, endNameAddr } = readName(data);
  const timingDataFromHeader = getTimingDataFromHeader(data, endNameAddr);
  const displayedMs = timingDataFromHeader.totalTimeToFinishMs - timingDataFromHeader.crossStartPlusStartDelayMs;

  const lagBeforeStartMs = countSame(data) * 10;
  const startMs = timingDataFromHeader.crossStartPlusStartDelayMs - lagBeforeStartMs;
  const finishDelayMs = timingDataFromHeader.totalRecordingTimeMs - timingDataFromHeader.totalTimeToFinishMs;

  const result: AnalyzeResult = {
    playerName,
    trackName: UNKNOWN_TRACK,

    // for FM Matt 54.35
    // 63.78 (total recording time)
    // 6.41  (cross start + start delay)
    // 26.65 (checkpoint 1 total time)
    // 60.76 (total time to finish)
    // 3.40 (startTime)
    displayedMs,
    startMs,
    totalMs: displayedMs + startMs,

    lagBeforeStartMs: lagBeforeStartMs,
    lagAfterFinishMs: finishDelayMs,
    recordingMs: timingDataFromHeader.totalRecordingTimeMs,

    checkpoint1Ms: timingDataFromHeader.checkpoint1TotalMs - timingDataFromHeader.crossStartPlusStartDelayMs,
    timingDataFromHeader,
    data
  };

  if (!options?.skipCoords) {
    const coords = getCoordinateData(data, timingDataFromHeader);
    result.coords = coords;
    const trackScoreData = getEveryLevelScored(result);
    result.trackScoreData = trackScoreData;
    // console.log(result.trackScoreData.everyLevelScored);
    // console.log(result.trackScoreData.allCollisions.filter(t => t.name === 'VillageHard')[0].collisions);
    // console.log(result.trackScoreData.everyLevelScored.filter(t => t.name === "AlpineMedium")[0].scoreData)
    const topScore = trackScoreData.levelScores[0].score;
    
    if (topScore !== 0) {
      const topScoring = trackScoreData.levelScores.filter(t => t.score === topScore).map(t => t.name).sort();
      if (topScoring.length === 2 && topScoring[0] === "ForestHard" && topScoring[1] === "ForestMedium") {
        result.trackName = "Forest MediumOrHard";
      } else if (topScoring.length === 1) {
        result.trackName = topScoring[0];
      } else {
        result.trackName = "UnknownTrack"
      }
    }
  }

  return result;
}

function readName(data: Uint8Array): { playerName: string; endNameAddr: number } {
  for (let i = 24; i < data.length; ++i) {
    if (data[i] === 0x00) {
      const nameBytes = data.slice(24, i);
      const playerName = new TextDecoder('utf-8').decode(nameBytes);
      return { playerName, endNameAddr: i };
    }
  }
  throw new Error("Player name not found in replay data");
}

function getTimingDataFromHeader(data: Uint8Array, endNameAddr: number): TimingDataFromHeader {
  // for FM Matt 54.35
  // 63.78 (total recording time)
  // 6.41  (cross start + start delay)
  // 26.65 (checkpoint 1 total time)
  // 60.76 (total time to finish)
  return {
    totalRecordingTimeMs: 10 * (data[4] | (data[5] << 8)),
    crossStartPlusStartDelayMs: 10 * (data[endNameAddr + 1] | (data[endNameAddr + 2] << 8)),
    checkpoint1TotalMs: 10 * (data[endNameAddr + 5] | (data[endNameAddr + 6] << 8)),
    totalTimeToFinishMs: 10 * (data[endNameAddr + 9] | (data[endNameAddr + 10] << 8)),
  };
}

function countSame(data: Uint8Array): number {
  const firstBlockOffset = getFirstBlockOffset(data);
  for (let i = firstBlockOffset, count = 1; i < data.length; i += BLOCK_SIZE, count++) {
    if (!blockIsSame(data, i, i + BLOCK_SIZE)) {
      return count;
    }
  }
  throw new Error("unreachable");
}

function blockIsSame(data: Uint8Array, block1: number, block2: number): boolean {
  for (let i = 0; i < BLOCK_SIZE; i++) {
    if (data[block1 + i] !== data[block2 + i]) {
      return false;
    }
  }
  return true;
}

const BLOCK_SIZE = 109;

function getFirstBlockOffset(data: Uint8Array): number {
  const { endNameAddr } = readName(data);
  return endNameAddr + 13;
}

function getTotalNumberOfBlocks(data: Uint8Array, firstBlockOffset: number): number {
  return 1 + ((data.length - firstBlockOffset) / BLOCK_SIZE);
}

function getCoordinateData(blockData: Uint8Array, timingDataFromHeader: TimingDataFromHeader): CoordinateData {
  const footerExists = blockData.slice(blockData.length - BLOCK_SIZE, BLOCK_SIZE).every(byte => byte === 0x00);
  const coordinateData: CoordinateData = { rows: [] };
  const dataView = new DataView(blockData.buffer, 0);

  const firstBlockOffset = getFirstBlockOffset(blockData);
  const totalCoordinateBlocks = getTotalNumberOfBlocks(blockData, firstBlockOffset) - 1 - (footerExists ? 1 : 0);
  const numExpectedBlocks = timingDataFromHeader.totalRecordingTimeMs / 10;
  if (totalCoordinateBlocks !== numExpectedBlocks) {
    console.warn(`Warning: Expected ${numExpectedBlocks} coordinate blocks, but got ${totalCoordinateBlocks}`);
  }

  const lastBlock = footerExists ? blockData.length - BLOCK_SIZE : blockData.length;
  for (let i = firstBlockOffset; i < lastBlock; i += BLOCK_SIZE) {
    const posOffset = i + 44;
    const matrixOffset = posOffset + 3 * 4;

    const rotation3x3: Array<Array<number>> = [];
    for (let row = 0; row < 3; row++) {
      const rowData: number[] = [];
      for (let col = 0; col < 3; col++) {
        const offset = matrixOffset + (row * 3 + col) * 4;
        rowData.push(dataView.getFloat32(offset, true));
      }
      rotation3x3.push(rowData);
    }
    const data = {
      x: dataView.getFloat32(posOffset, true),
      y: dataView.getFloat32(posOffset + 4, true),
      z: dataView.getFloat32(posOffset + 2 * 4, true),
      rotation3x3,
    };
    coordinateData.rows.push(data);
  }
  return coordinateData;
}

export function getBlock(data: Uint8Array, blockIndex: number): Uint8Array {
  const firstBlockOffset = getFirstBlockOffset(data);
  const blockStart = firstBlockOffset + blockIndex * BLOCK_SIZE;
  const blockEnd = blockStart + BLOCK_SIZE;
  if (blockEnd > data.length) {
    throw new Error(`Block index ${blockIndex} is out of bounds`);
  }
  return data.slice(blockStart, blockEnd);
}