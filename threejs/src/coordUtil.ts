import { RowData } from "analyze/src/types";

export function calculateAcceleration(data: RowData[], currentIndex: number): number {
  if (currentIndex < 1 || currentIndex >= data.length - 1) return 0;
  
  const dt = 1/100;
  const prevY = data[currentIndex - 1].y;
  const currY = data[currentIndex].y;
  const nextY = data[currentIndex + 1].y;
  
  return (nextY - 2*currY + prevY) / (dt * dt);
}


export function getSpeed(
  data: Array<RowData>,
  frameToRender: number,
  frameRate: number = 100
) {
  if (frameToRender === 0) return 0;

  const r0 = data[frameToRender - 1];
  const r1 = data[frameToRender];

  const dx = r1.x - r0.x;
  const dy = r1.y - r0.y;
  const dz = r1.z - r0.z;
  const distance = Math.sqrt(dx * dx + dy * dy + dz * dz);

  const timeBetweenFramesHours = 1 / (frameRate * 3600);
  const speed = distance / 1000 / timeBetweenFramesHours;

  return speed;
}