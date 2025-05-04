import { invoke } from "@tauri-apps/api/core";

export async function checkForUpdates(): Promise<{ currentVersion: string; latestVersion: string; }> {
  return await invoke('check_for_updates');
}

// use tauri invoke