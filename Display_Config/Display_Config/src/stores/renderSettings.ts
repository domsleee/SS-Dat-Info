import { defineStore } from 'pinia';
import { ref, type Ref } from 'vue';
import type { RenderSettings } from '../services/types';
import { getPersistentSettings } from './persistentStoreHelper';
import type { DetailConfig, RdConfig } from './types';
import { tryParseInt } from '@/services/stringUtil';


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
    renderDistance: 450,
    groundDetail: 3,
  });
  return { renderSettings: settings };
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

export function getAsDetailConfig(): DetailConfig {
  const { renderSettings } = useRenderSettingsStore();
  return {
    renderDistance: tryParseInt(renderSettings.renderDistance),
    groundDetail: tryParseInt(renderSettings.groundDetail),
  }
}
