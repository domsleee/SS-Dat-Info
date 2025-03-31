import { getEveryLevelScored } from "./PlaneUtil/scoreTrack";
import { AnalyzeReplayOptions, AnalyzeResult, CoordinateData, TimingDataFromHeader, UNKNOWN_TRACK } from "./types";

export function analyzeReplayHex(hexData: string, options?: AnalyzeReplayOptions): AnalyzeResult {
  const { playerName, endNameAddr } = readName(hexData);
  const timingDataFromHeader = getTimingDataFromHeader(hexData, endNameAddr);
  const displayedMs = timingDataFromHeader.totalTimeToFinishMs - timingDataFromHeader.crossStartPlusStartDelayMs;

  const lagBeforeStartMs = countSame(hexData) * 10;
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
    timingDataFromHeader
  };

  if (!options?.skipCoords) {
    const coords = getCoordinateData(hexData, timingDataFromHeader);
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

function readName(hexData: string): { playerName: string; endNameAddr: number } {
  // read bytes from 48 until hitting "00"
  let s = "";
  let playerName = "";
  let i = 48;
  for (; i < hexData.length; i += 2) {
    if (hexData.slice(i, i + 2) === "00") {
      playerName = Buffer.from(s, "hex").toString("utf8");
      break;
    }
    s += hexData.slice(i, i + 2);
  }
  return { playerName, endNameAddr: i };
}

function getTimingDataFromHeader(hexData: string, endNameAddr: number): TimingDataFromHeader {
  // for FM Matt 54.35
  // 63.78 (total recording time)
  // 6.41  (cross start + start delay)
  // 26.65 (checkpoint 1 total time)
  // 60.76 (total time to finish)
  return {
    totalRecordingTimeMs: 10 * parseLittleEndian16(hexData.slice(8, 12)),
    crossStartPlusStartDelayMs: 10 * parseLittleEndian16(hexData.slice(endNameAddr + 2, endNameAddr + 6)),
    checkpoint1TotalMs: 10 * parseLittleEndian16(hexData.slice(endNameAddr + 10, endNameAddr + 14)),
    totalTimeToFinishMs: 10 * parseLittleEndian16(hexData.slice(endNameAddr + 18, endNameAddr + 22)),
  };
}

function parseLittleEndian16(hexData: string) {
  // 16-bit little endian. e.g. ea18 is 6378
  const low = parseInt(hexData.slice(0, 2), 16);
  const high = parseInt(hexData.slice(2, 4), 16);
  return (high << 8) | low;
}

export function parseLittleEndianFloat32Old(hexData: string) {
  // 32-bit little endian. e.g. face0144 is 519.234
  return new Float32Array(new Uint32Array([parseInt(hexData.match(/../g)!.reverse().join(""), 16)]).buffer)[0];
}

const u32Array = new Uint32Array(1);
const f32Array = new Float32Array(u32Array.buffer);

export function parseLittleEndianFloat32(hexData: string): number {
  let reversedHex = "";
  for (let i = hexData.length - 2; i >= 0; i -= 2) {
    reversedHex += hexData.substring(i, i + 2);
  }
  
  u32Array[0] = parseInt(reversedHex, 16);
  return f32Array[0];
}

function countSame(hexData: string): number {
  const blocks = getDataBlocks(hexData);
  for (let i = 1; i < blocks.length - 1; ++i) {
    if (blocks[i] !== blocks[i + 1]) {
      return i;
    }
  }
  throw new Error("unreachable");
}

export function getDataBlocks(hex: string): Array<string> {
  const { endNameAddr } = readName(hex);
  const offset = 26;
  const blocks = [hex.slice(0, endNameAddr + offset)];
  for (let i = endNameAddr + offset; i < hex.length; i += 218) {
    blocks.push(hex.slice(i, i + 218));
  }
  return blocks;
}

function getCoordinateData(hex: string, timingDataFromHeader: TimingDataFromHeader): CoordinateData {
  const blocks = getDataBlocks(hex);
  const footerExists = blocks.at(-1)?.match(/^[0]+$/);
  const coordinateData: CoordinateData = { rows: [] };

  const totalCoordinateBlocks = blocks.length - 1 - (footerExists ? 1 : 0);
  const numExpectedBlocks = timingDataFromHeader.totalRecordingTimeMs / 10;
  if (totalCoordinateBlocks !== numExpectedBlocks) {
    console.warn(`Warning: Expected ${numExpectedBlocks} coordinate blocks, but got ${totalCoordinateBlocks}`);
  }

  for (let index = 1; index < blocks.length - (footerExists ? 1 : 0); ++index) {
    const block = blocks[index];
    const posOffset = 88;
    const matrixOffset = posOffset + 3 * 8;

    const rotation3x3: Array<Array<number>> = [];
    for (let row = 0; row < 3; row++) {
      const rowData: number[] = [];
      for (let col = 0; col < 3; col++) {
        const offset = matrixOffset + (row * 3 + col) * 8;
        rowData.push(parseLittleEndianFloat32(block.slice(offset, offset + 8)));
      }
      rotation3x3.push(rowData);
    }
    const data = {
      x: parseLittleEndianFloat32(block.slice(posOffset, posOffset + 8)),
      y: parseLittleEndianFloat32(block.slice(posOffset + 8, posOffset + 2 * 8)),
      z: parseLittleEndianFloat32(block.slice(posOffset + 2 * 8, posOffset + 3 * 8)),
      rotation3x3,
      ex: block.slice(202, 208),
      raw: block
    };
    coordinateData.rows.push(data);
  }
  return coordinateData;
}
