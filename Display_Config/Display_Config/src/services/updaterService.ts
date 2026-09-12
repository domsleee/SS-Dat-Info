import { commands, type DownloadEvent } from "@/bindings";
import { runWithErrorHandler } from "@/stores/errorStore";
import { useUpdateDialogStore } from "@/stores/updateDialogStore";
import { Channel } from "@tauri-apps/api/core";

export async function checkForUpdates(force = false): Promise<{ currentVersion: string; latestVersion: string; }> {
  return await commands.checkForUpdates(force);
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
        if (state.state.cancelling) void commands.cancelDownload(message.data.token);
      }
      if (message.event === 'installing') {
        state.state = { key: 'installing', latestVersion };
      }
      if (message.event === 'downloadProgress' && state.state.key === 'downloading') {
        const { progressTotal, total } = message.data;
        state.state.progress = total > 0 ? Math.round(progressTotal / total * 100) : 0;
      }
    });
    let installed: boolean;
    try {
      ({ installed } = await commands.downloadAndExtract(url, onEvent));
    } finally {
      state.state = { key: 'closed' };
    }
    if (!installed) return;

    state.state = {
      key: 'finished',
      latestVersion,
    };
    await commands.relaunch();
  });
}
