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

export async function runWithErrorHandler<T>(asyncFn: () => Promise<T>): Promise<T | undefined> {
  try {
    return await asyncFn();
  } catch (e) {
    const errorStore = useErrorStore();
    if (e instanceof Error || typeof e === 'string') {
      errorStore.setError(e);
    } else {
      errorStore.setError('Unknown error');
    }
  }
}

interface Options {
  logData: string[];
}