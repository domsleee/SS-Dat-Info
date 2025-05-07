import { defineStore } from "pinia";

export const useUpdateDialogStore = defineStore(
  'updateDialogStore',
  {
    state: (): { state: State } => ({
      state: { key: 'closed' }
    }),
  }
);

type State = 
  | { key: 'closed' }
  | { key: 'checking'}
  | {
    key: 'updateAvailable';
    currentVersion: string;
    latestVersion: string;
  }
  | {
    key: 'noUpdateAvailable';
    currentVersion: string;
    latestVersion: string;
  }
  | {
    key: 'downloading';
    latestVersion: string;
    progress: number;
    token?: string;
  }
  | {
    key: 'finished';
    latestVersion: string;
  };