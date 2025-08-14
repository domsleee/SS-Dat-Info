import { AnalyzeResult } from "dat-analyze/src/types";
import { getHumanReadableTime, msToHumanReadable } from "./timeUtil";

export function setupExportToCsvButton(button: HTMLElement, analyzeResult: AnalyzeResult) {
  button.onclick = () => {
    const rows = [
      ['x', 'y', 'z', 'timer']
    ];
    for (const [i, row] of analyzeResult.coords?.rows.entries() || []) {
      const time = getHumanReadableTime(analyzeResult, i);
      rows.push([row.x.toString(), row.y.toString(), row.z.toString(), time]);
    }

    const csv = rows.map(r => r.join(',')).join('\n');
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = url;
    a.download = getCsvName(analyzeResult);
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
}

function getCsvName(analyzeResult: AnalyzeResult) {
  const { timingDataFromHeader } = analyzeResult;
  const didFinishRun = timingDataFromHeader.totalTimeToFinishMs !== 0;
  if (!didFinishRun) {
    return 'export.csv';
  }

  const time = msToHumanReadable(analyzeResult.displayedMs).replaceAll(':', '.').replace(/^0+/, '');

  return `${time} ${analyzeResult.playerName}.csv`
}