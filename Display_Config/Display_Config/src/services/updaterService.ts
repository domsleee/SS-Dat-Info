import { commands, type DownloadEvent } from "@/bindings";
import { runWithErrorHandler } from "@/stores/errorStore";
import { useUpdateDialogStore } from "@/stores/updateDialogStore";
import { Channel } from "@tauri-apps/api/core";

export async function checkForUpdates(): Promise<{ currentVersion: string; latestVersion: string; }> {
  return (await commands.checkForUpdates());
}

export async function update(latestVersion: string): Promise<void> {
  const state = useUpdateDialogStore();
  state.state = {
    key: 'downloading',
    latestVersion,
    progress: 0,
  }
  await runWithErrorHandler(async () => {
    if (state.state.key !== 'downloading') return;
    const url = `https://github.com/domsleee/SS-Dat-Info/releases/download/${state.state.latestVersion}/Display_Config_${state.state.latestVersion}.zip`;

    const onEvent = new Channel<DownloadEvent>((message) => {
      if (state.state.key === 'downloading' && message.event === 'token') {
        state.state.token = message.data.token;
      }
      if (message.event === 'downloadProgress' && state.state.key === 'downloading') {
        const { progressTotal, total } = message.data;
        state.state.progress = parseFloat(((progressTotal / total) * 100).toFixed(0));
      }
    });
    const { installed } = await commands.downloadAndExtract(url, onEvent);
    if (!installed) return;

    state.state = {
      key: 'finished',
      latestVersion,
    };
    commands.relaunch();
  });
}
