import { CircleEntry } from "./generateCircle";

export function circleToString(circle: Array<CircleEntry>): string {
  return circle.map((entry) => {
    return `${entry.name} {
uniform_scale = ${entry.uniform_scale.toFixed(3)};
loc = {${entry.loc.x}, ${entry.loc.y}, ${entry.loc.z}};
level_inclusion_mask = -1;
offset = {${entry.offset.x.toFixed(3)}, ${entry.offset.y.toFixed(3)}, ${entry.offset.z.toFixed(3)}};
quat = {${entry.quat.w.toFixed(6)}, {${entry.quat.x.toFixed(6)}, ${entry.quat.y.toFixed(6)}, ${entry.quat.z.toFixed(6)}}};
melt_height = 0;
};`}).join("\n\n");
}
