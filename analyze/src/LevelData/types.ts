import { PositionXYZ, Quaternion } from "../generateCircle/types";

export interface LevelDataEntry {
    name: string;
    uniform_scale: number;
    position: PositionXYZ;
    quat: Quaternion;
}
