import type { FovType } from "./fovOptions";

export interface Config {
  renderSettings: RenderSettings;
  trainerUISettings: TrainerUISettings;
}

export interface RenderSettings {
  renderer?: string;
  cardId?: number;
  resolution?: string;
  colourDepth?: string;
  fullscreen: boolean;
  language: string;
  renderDistance: number;
  groundDetail: number;
  ghostPlayer: string;
}

export interface TrainerUISettings {
  use4xFonts: boolean;
  fovType: FovType;
  fovWidth?: number;
  fovHeight?: number;
  enableLogging: boolean;
  makeGhostsOpaque: boolean;
  matchGhostSoundsToCharacter: boolean;
}
