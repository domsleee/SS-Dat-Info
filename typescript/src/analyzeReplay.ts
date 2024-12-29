import { readFile } from "fs/promises";
import { AnalyzeResult, TimingDataFromHeader } from "./types";

const BLOCK_SEP = "00003f48e1fa3e00000000";
export async function analyzeReplay(filepath: string): Promise<AnalyzeResult> {
  const content = await readFile(filepath);
  const hexData = Buffer.from(content.buffer).toString("hex");

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
  };
}

function getTrackName(hexData: string): string {
  const firstBlock = hexData.split(BLOCK_SEP).at(1)!;

  const prefixes = {
    "Alpine EasyMediumOrHard": "81ad2044d1",
    "Forest Easy": "1b97034427",
    "Forest MediumOrHard": "face01443",
    "Village Easy": "cd94374498",
    "Village Medium": "b87e49443b",
    "Village Hard": "c1aad84308",
  };

  for (const [k, v] of Object.entries(prefixes)) {
    if (firstBlock.startsWith(v)) {
      return k;
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

function countSame(hexData: string): number {
  const blocks = getDataBlocks(hexData);
  for (let i = 1; i < blocks.length - 1; ++i) {
    if (blocks[i] !== blocks[i + 1]) {
      return 1 + i;
    }
  }
  throw new Error("unreachable");
}

function getDataBlocks(hex: string): Array<string> {
  const spl = hex.split(BLOCK_SEP);
  const s = spl.slice(1).join(BLOCK_SEP);
  if (!s) throw new Error("Corrupt, no separator found");
  const blocks = [spl[0]];
  for (let i = 0; i < s.length; i += 218) {
    blocks.push(s.slice(i, i + 218));
  }
  return blocks;
}
