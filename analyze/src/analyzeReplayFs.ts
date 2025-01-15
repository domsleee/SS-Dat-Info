import { readFile } from "fs/promises";
import { analyzeReplayHex } from "./analyzeReplay";
import { AnalyzeResult } from "./types";

export async function analyzeReplay(filepath: string, includeCoords?: boolean): Promise<AnalyzeResult> {
  const content = await readFile(filepath);
  const hexData = Buffer.from(content.buffer).toString("hex");
  return analyzeReplayHex(hexData, includeCoords);
}
