import { AnalyzeResult } from "dat-analyze/src/types";


export function getHumanReadableTime(analyzeResult: AnalyzeResult, frame: number) {
  let actualMs = Math.max(
    0,
    10 * frame - (analyzeResult.lagBeforeStartMs + analyzeResult.startMs)
  );
  actualMs = Math.min(actualMs, analyzeResult.displayedMs);
  return msToHumanReadable(actualMs);
}

export function msToHumanReadable(ms: number) {
  return new Date(ms).toISOString().slice(14, 22).replace(".", ":");
}