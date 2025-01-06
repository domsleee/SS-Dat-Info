import { classifyMessage } from "./classifyMessage";
import { AnalyzeResult, CoordinateData, MovementState, TimingDataFromHeader } from "./types";

export function analyzeReplayHex(hexData: string, includeCoords?: boolean): AnalyzeResult {
  const trackName = getTrackName(hexData);

  const { playerName, endNameAddr } = readName(hexData);
  const timingData = getTimingData(hexData, endNameAddr);
  const displayedMs = timingData.totalTimeToFinishMs - timingData.crossStartPlusStartDelayMs;

  const lagBeforeStartMs = countSame(hexData) * 10;
  const startMs = timingData.crossStartPlusStartDelayMs - lagBeforeStartMs;
  const finishDelayMs = timingData.totalRecordingTimeMs - timingData.totalTimeToFinishMs;

  return {
    playerName,
    trackName,

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
    recordingMs: timingData.totalRecordingTimeMs,

    checkpoint1Ms: timingData.checkpoint1TotalMs - timingData.crossStartPlusStartDelayMs,

    ...(includeCoords ? { coords: getCoordinateData(hexData) } : {}),
  };
}

function getTrackName(hexData: string): string {
  const prefixes = {
    "Alpine EasyMediumOrHard": "81ad2044d1a452c49585c342e9fc",
    "Forest Easy": "1b9703442763afc497505342f8ff",
    "Forest MediumOrHard": "face01443167c2c4df4f56427eff",
    "Forest MediumOrHard v2": "face01443267c2c4df4f56427dff",
    "Village Easy": "cd9437449815e7c333d36143fcff",
    "Village Easy v2": "cd9437449815e7c333d36143fdff",
    "Village Medium": "b87e49443ba4f2c3e1ba9143ffff",
    "Village Medium v2": "b87e49443ba4f2c3e1ba9143feff",
    "Village Hard": "c1aad843088b00c44a7bc04211dc",
    "Village Hard v2": "c1aad843088b00c44a7bc04214dc",
  };

  const firstBlock = getDataBlocks(hexData)[1];
  for (const [k, v] of Object.entries(prefixes)) {
    const sli = firstBlock.slice(88, 88 + 28);
    if (sli === v.slice(0, 28)) {
      return k.replace(" v2", "").replace(" v3", "").replace(" v4", "");
    }
  }

  throw new Error("track name could not be determined");
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

function getTimingData(hexData: string, endNameAddr: number): TimingDataFromHeader {
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

function parseLittleEndianFloat32(hexData: string) {
  // 32-bit little endian. e.g. face0144 is 519.234
  return new Float32Array(new Uint32Array([parseInt(hexData.match(/../g)!.reverse().join(""), 16)]).buffer)[0];
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
  const { playerName, endNameAddr } = readName(hex);
  const offset = 26;
  const blocks = [hex.slice(0, endNameAddr + offset)];
  for (let i = endNameAddr + offset; i < hex.length; i += 218) {
    blocks.push(hex.slice(i, i + 218));
  }
  return blocks;
}

function getCoordinateData(hex: string): CoordinateData {
  const blocks = getDataBlocks(hex);
  const coordinateData: CoordinateData = { rows: [] };
  for (let index = 1; index < blocks.length; ++index) {
    const block = blocks[index];
    const offset = 88;
    const state = classifyMessage(block);
    const data = {
      x: parseLittleEndianFloat32(block.slice(offset, offset + 8)),
      y: parseLittleEndianFloat32(block.slice(offset + 8, offset + 2 * 8)),
      z: parseLittleEndianFloat32(block.slice(offset + 2 * 8, offset + 3 * 8)),
      rx: parseLittleEndianFloat32(block.slice(offset + 3 * 8, offset + 4 * 8)),
      rw: parseLittleEndianFloat32(block.slice(offset + 4 * 8, offset + 5 * 8)),
      ry: parseLittleEndianFloat32(block.slice(offset + 5 * 8, offset + 6 * 8)),
      rz: parseLittleEndianFloat32(block.slice(offset + 6 * 8, offset + 7 * 8)),
      ...state,
      ex: block.slice(202, 208),
      raw: block
    };
    // if (index === 302) {
    //   console.log(block);
    //   console.log("SKIB", block[214])
    //   console.log(block.slice(214));
    //   console.log(data, coordinateData.rows.length)
    // }
    coordinateData.rows.push(data);
  }
  return coordinateData;
}


// 1: 3
// 37: 75
