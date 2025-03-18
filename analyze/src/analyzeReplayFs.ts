import { readFile } from "fs/promises";
import { analyzeReplayHex } from "./analyzeReplay";
import { AnalyzeReplayOptions, AnalyzeResult } from "./types";

export async function analyzeReplay(filepath: string, options?: AnalyzeReplayOptions): Promise<AnalyzeResult> {
  const content = await readFile(filepath);
  const hexData = Buffer.from(content.buffer).toString("hex");
  return analyzeReplayHex(hexData, options);
}
