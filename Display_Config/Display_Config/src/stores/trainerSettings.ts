import { defineStore } from "pinia";
import { ref } from "vue";
import type { TrainerSettings } from "../services/types";

export const useTrainerSettingsStore = defineStore(
  "trainerForm",
  () => {
    const settings = ref<TrainerSettings>({
      changeFov: true,
      use4xFonts: false,
      fovWidth: 1920,
      fovHeight: 1080,
    });
  
    return { trainerSettings: settings }
  },
  // @ts-expect-error will fix
  { persist: true }
);