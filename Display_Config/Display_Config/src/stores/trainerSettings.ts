import { defineStore } from "pinia";
import { ref, type Ref } from "vue";
import type { TrainerSettings } from "../services/types";
import { getPersistentSettings } from './persistentStoreHelper';

export const useTrainerSettingsStore = defineStore(
  "trainerForm",
  () => getDefaultTrainerSettings(),
  getPersistentSettings(getDefaultTrainerSettings)
);

function getDefaultTrainerSettings(): { trainerSettings: Ref<TrainerSettings> } {
  const settings = ref<TrainerSettings>({
    changeFov: false,
    use4xFonts: false,
    fovWidth: 1920,
    fovHeight: 1080,
    enableLogging: false,
  });
  return { trainerSettings: settings };
}
