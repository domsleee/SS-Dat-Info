import { readFile } from "fs/promises";
import { analyzeReplay } from "./analyzeReplay";
import { AnalyzeReplayOptions, AnalyzeResult } from "./types";

export async function analyzeReplayFile(filepath: string, options?: AnalyzeReplayOptions): Promise<AnalyzeResult> {
  const content = await readFile(filepath);
  return analyzeReplay(new Uint8Array(content), options);
}
