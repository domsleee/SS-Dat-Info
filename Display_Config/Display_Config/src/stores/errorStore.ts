import { defineStore } from "pinia";

export const useErrorStore = defineStore("errorStore", {
  state: () => ({
    title: "",
    error: "" as string | Error,
    options: undefined as Options | undefined,
    show: false,
  }),
  actions: {
    setError(error: Error | string, options?: Options) {
      this.error = error;
      this.options = options;
      this.show = true;
    },
    clearError() {
      this.show = false;
    },
  },
});

interface Options {
  logData: string[];
}