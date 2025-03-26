import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';
import type { RenderSettings } from '../services/types';
import { getPersistentSettings } from './persistentStoreHelper';

export const useRenderSettingsStore = defineStore(
  'renderSettings',
  getDefaultRenderSettings,
  getPersistentSettings(getDefaultRenderSettings)
);

function getDefaultRenderSettings(): { renderSettings: Ref<RenderSettings> } {
  const settings = ref<RenderSettings>({
    renderer: "OpenGL",
    cardId: 0,
    resolution: "1600x1200",
    colourDepth: "32bit",
    fullscreen: false,
    language: "English",
  });
  return { renderSettings: settings };
}

interface RdConfig {
  apiName: string;
  width: number;
  height: number;
  depth: number;
  cardId: number;
  fullscreen: boolean;
}

export function getAsRdConfig(): RdConfig {
  const { renderSettings } = useRenderSettingsStore();
  return {
    apiName: renderSettings.renderer!,
    width: parseInt(renderSettings.resolution!.split('x')[0]),
    height: parseInt(renderSettings.resolution!.split('x')[1]),
    depth: parseInt(renderSettings.colourDepth!.split('bit')[0]),
    cardId: renderSettings.cardId!,
    fullscreen: renderSettings.fullscreen,
  }
}
