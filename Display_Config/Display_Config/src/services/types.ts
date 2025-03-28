export interface Config {
  renderSettings: RenderSettings;
  trainerSettings: TrainerSettings;
}

export interface RenderSettings {
  renderer?: string;
  cardId?: number;
  resolution?: string;
  colourDepth?: string;
  fullscreen: boolean;
  language: string;
}

export interface TrainerSettings {
  use4xFonts: boolean;
  changeFov: boolean;
  fovWidth?: number;
  fovHeight?: number;
  enableLogging: boolean;
  makeGhostsOpaque: boolean;
}
