import { defineStore } from "pinia";
import { ref } from "vue";

export const useAutostartStore = defineStore(
  'autostart',
  () => {
    const autostart = ref<boolean>(false);
    return { autostart };
  },
  // @ts-expect-error will fix
  { persist: true }
);