import { MovementState } from "./types";

export function classifyMessage(block: string): {
  movementState: MovementState,
  left: boolean,
  right: boolean,
  shift: boolean,
  forward: boolean,
} {
  const right = block[214] === '3';
  const left = block[214] === 'b';
  const shift = (parseInt(block[215], 16) & 0b1) === 0b1;
  const forward = false;

  return {
    movementState: "??",
    left,
    right,
    shift,
    forward,
  };
}
