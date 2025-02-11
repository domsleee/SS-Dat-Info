import { defineStore } from "pinia";
import { ref, type Ref } from "vue";
import { getPersistentSettings } from "./persistentStoreHelper";

export const useAutostartStore = defineStore(
  'autostart',
  getDefaultAutoStartSettings,
  getPersistentSettings(getDefaultAutoStartSettings)
);

function getDefaultAutoStartSettings(): { settings: Ref<AutostartSettings> } {
  return {settings: ref({
    autostart: false,
    autoCloseOthers: false
  })};
}

interface AutostartSettings {
  autostart: boolean;
  autoCloseOthers: boolean;
}
