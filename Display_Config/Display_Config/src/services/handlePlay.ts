
import { useTrainerUISettingsStore } from '../stores/trainerSettings';
import { exit } from '@tauri-apps/plugin-process';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getAsRdConfig, getAsDetailConfig, useRenderSettingsStore } from '../stores/renderSettings';
import { invoke } from '@tauri-apps/api/core';
import type { Ref } from 'vue';
import { useErrorStore } from '@/stores/errorStore';
import type { TrainerUISettings } from './types';
import { tryParseInt } from '../services/stringUtil';

export async function handlePlayAsync(playLoading: Ref<boolean>) {
  const errorStore = useErrorStore();
  console.log(useRenderSettingsStore().renderSettings);
  playLoading.value = true;
  try {
    console.log(await invoke('read_rd_config'));
    console.log(await writeDetailConfig());
    console.log(await invoke('write_rd_config', { rdConfig: getAsRdConfig() }));
    console.log(await invoke('write_language', { language: useRenderSettingsStore().renderSettings.language }));
    const trainerSettings = getTrainerSettings();
    if (requiresInject(trainerSettings)) {
      const r2 = await invoke('run_inject', { trainerSettings });
      console.log(r2);
    }
    await getCurrentWindow().setFocus();
    playLoading.value = false;
    await exit(0);
  } catch (e) {
    if (e instanceof Error || typeof e === 'string') {
      const errorString = e instanceof Error ? e.message : e;
      if (errorString.startsWith(`Timeout waiting for 'Finished.' in log`)) {
        const logData = await invoke<string[]>('get_log_data');
        errorStore.setError(e, {logData});
      } else {
        errorStore.setError(e);
      }
    } else {
      alert(e);
    }
    playLoading.value = false;
  }
}

export async function writeDetailConfig() {
  return await invoke('write_detail_config', { detailConfig: getAsDetailConfig() })
}

function requiresInject(trainerSettings: TrainerSettings): boolean {
  return trainerSettings.changeFov || trainerSettings.use4xFonts || trainerSettings.enableLogging || trainerSettings.makeGhostsOpaque || trainerSettings.matchGhostSoundsToCharacter;
}

function getTrainerSettings(): TrainerSettings {
  const { trainerSettings } = useTrainerUISettingsStore();
  const changeFov = shouldChangeFov(trainerSettings);

  return {
    use4xFonts: trainerSettings.use4xFonts,
    changeFov,
    fovWidth: tryParseInt(trainerSettings.fovWidth),
    fovHeight: tryParseInt(trainerSettings.fovHeight),
    enableLogging: trainerSettings.enableLogging,
    makeGhostsOpaque: trainerSettings.makeGhostsOpaque,
    matchGhostSoundsToCharacter: trainerSettings.matchGhostSoundsToCharacter,
  };
}

function shouldChangeFov(trainerSettings: TrainerUISettings): boolean {
  if (trainerSettings.fovType === "Original") return false;

  const fovWidth = tryParseInt(trainerSettings.fovWidth);
  const fovHeight = tryParseInt(trainerSettings.fovHeight);
  if (!fovWidth || !fovHeight) return false;

  // 4/3 = width/height
  if (4 * fovHeight === 3 * fovWidth) return false;
  return true;
}


interface TrainerSettings {
  use4xFonts: boolean;
  changeFov: boolean;
  fovWidth?: number;
  fovHeight?: number;
  enableLogging: boolean;
  makeGhostsOpaque: boolean;
  matchGhostSoundsToCharacter: boolean;
}