import { analyzeReplay } from "dat-analyze/src/analyzeReplay";
import { AnalyzeResult } from "dat-analyze/src/types";
export async function getDomAH() {
  return await getFromFilename("replays/Alpine/Hard/1.07.69 Dom.dat");
}

export async function getDomVM() {
  return await getFromFilename("replays/Village/Medium/VM 1.09.08 Dom.dat");
}

export async function getFromFilename(filename: string): Promise<AnalyzeResult> {
  const response = await fetch(filename);
  const arrayBuffer = await response.arrayBuffer();
  const replayData = analyzeReplay(new Uint8Array(arrayBuffer));
  return replayData;
}
