export type Vector = Vector[] | string;
export type GameValue = Vector | string;

export interface GameObject {
  type: string;
  properties: Record<string, GameValue>;
}

export interface ParserPosition {
  pos: number;
  line: number;
  current: string | null;
}