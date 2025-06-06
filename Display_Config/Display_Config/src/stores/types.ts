export interface RdConfig {
  apiName: string;
  width: number;
  height: number;
  depth: number;
  cardId: number;
  fullscreen: boolean;
}

export interface DetailConfig {
  renderDistance: number | null;
  groundDetail: number | null;
}
