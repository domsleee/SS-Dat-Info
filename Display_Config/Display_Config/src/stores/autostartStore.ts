import { defineStore } from "pinia";
import { ref } from "vue";

export const useAutostartStore = defineStore(
  'autostart',
  () => {
    const autostart = ref<boolean>(false);
    return { autostart };
  },
  { persist: true }
);