
import { useTrainerUISettingsStore } from '../stores/trainerSettings';
import { exit } from '@tauri-apps/plugin-process';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getAsRdConfig, useRenderSettingsStore } from '../stores/renderSettings';
import { invoke } from '@tauri-apps/api/core';
import type { Ref } from 'vue';
import { useErrorStore } from '@/stores/errorStore';
import type { TrainerUISettings } from './types';


export async function handlePlayAsync(playLoading: Ref<boolean>) {
  const errorStore = useErrorStore();
  console.log(useRenderSettingsStore().renderSettings);
  playLoading.value = true;
  try {
    console.log (await invoke('read_rd_config'));
    console.log(await invoke('write_rd_config', { config: getAsRdConfig() }));
    console.log(await invoke('write_language', { language: useRenderSettingsStore().renderSettings.language }));
    const trainerJsonSettings = getTrainerSEttings();
    if (requiresInject(trainerJsonSettings)) {

      const r2 = await invoke('run_inject', { trainerSettings: trainerJsonSettings });
      console.log(r2);
    }
    playLoading.value = false;
    await getCurrentWindow().setFocus();
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

function requiresInject(trainerSettings: TrainerSettings): boolean {
  return trainerSettings.changeFov || trainerSettings.use4xFonts || trainerSettings.enableLogging || trainerSettings.makeGhostsOpaque || trainerSettings.matchGhostSoundsToCharacter;
}

function getTrainerSEttings(): TrainerSettings {
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

function tryParseInt(value: string | number | undefined): number | undefined {
  if (value === undefined) return undefined;
  const parsedValue = parseInt((value as unknown as string) ?? '');
  return isNaN(parsedValue) ? undefined : parsedValue;
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