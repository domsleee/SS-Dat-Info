import { defineStore } from "pinia";
import { ref, type Ref } from "vue";
import type { TrainerUISettings } from "../services/types";
import { getPersistentSettings } from './persistentStoreHelper';

export const useTrainerUISettingsStore = defineStore(
  "trainerForm",
  getDefaultTrainerUISettings,
  getPersistentSettings(getDefaultTrainerUISettings)
);

function getDefaultTrainerUISettings(): { trainerSettings: Ref<TrainerUISettings> } {
  const settings = ref<TrainerUISettings>({
    fovType: "MatchRes",
    use4xFonts: false,
    fovWidth: 1920,
    fovHeight: 1080,
    enableLogging: false,
    makeGhostsOpaque: false,
    matchGhostSoundsToCharacter: false,
  });
  return { trainerSettings: settings };
}
