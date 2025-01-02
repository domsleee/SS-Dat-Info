import { analyzeReplayHex } from "analyze/src/analyzeReplay";
export async function getDomAH() {
  return await getFromFilename("replays/Alpine/Hard/1.07.69 Dom.dat");
}

export async function getDomVM() {
  return await getFromFilename("replays/Village/Medium/VM 1.05.13 Dom.dat");
}

async function getFromFilename(filename: string) {
  const response = await fetch(filename);
  const arrayBuffer = await response.arrayBuffer();
  const hexData = Buffer.from(arrayBuffer).toString("hex");
  const replayData = analyzeReplayHex(hexData, true);
  return replayData.coords!.rows.map((row) => row);
}
