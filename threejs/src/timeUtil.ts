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

export function msToHumanReadableWithMs(ms: number) {
  const DP = 4;
  const pow10 = Math.pow(10, DP-3);
  const intMs = Math.trunc(ms % 1000);
  let nDigits = intMs * pow10;
  if (DP > 3) {
    nDigits += Math.trunc((ms % 1) * pow10);
  }

  return new Date(ms).toISOString().slice(14, 20)
    + String(nDigits).padStart(DP, '0');
}