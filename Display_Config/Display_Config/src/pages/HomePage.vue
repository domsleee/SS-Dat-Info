<template>
  <v-container style="max-width: 400px">
    <v-form :disabled="playLoading">
      <RenderOptions :formIsLoading="playLoading" />
      <TrainerOptions :formIsLoading="playLoading" class="mt-2" />
      <Autostart @auto-play="handleAutoplay()" />
      <div class="mt-2 ga-2 d-flex justify-end">
        <v-btn size="x-large" color="indigo-darken-3" :loading="playLoading" @click="handlePlay">Play</v-btn>
        <v-btn size="x-large" color="" @click="handleExit">Exit</v-btn>
      </div>
      <ErrorDialog />
    </v-form>
  </v-container>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getAsRdConfig, useRenderSettingsStore } from '@/stores/renderSettings';
import Autostart from '@/components/Autostart.vue';
import { useTrainerSettingsStore } from '@/stores/trainerSettings';
import { exit } from '@tauri-apps/plugin-process';
import { useErrorStore } from '@/stores/errorStore';

const playLoading = ref(false);
const errorStore = useErrorStore();

async function handleAutoplay() {
  if (!playLoading.value) {
    await handlePlay();
  }
}

async function handlePlay() {
  console.log(useRenderSettingsStore().renderSettings);
  playLoading.value = true;
  try {
    console.log (await invoke('read_rd_config'));
    console.log(await invoke('write_rd_config', { config: getAsRdConfig() }));
    console.log(await invoke('write_language', { language: useRenderSettingsStore().renderSettings.language }));
    const { trainerSettings } = useTrainerSettingsStore();
    if (trainerSettings.changeFov || trainerSettings.use4xFonts || trainerSettings.enableLogging || trainerSettings.makeGhostsOpaque) {
      if (typeof trainerSettings.fovHeight === 'string') trainerSettings.fovHeight = parseInt(trainerSettings.fovHeight);
      if (typeof trainerSettings.fovWidth === 'string') trainerSettings.fovWidth = parseInt(trainerSettings.fovWidth);
      const r2 = await invoke('run_inject', { trainerSettings });
      console.log(r2);
    }
    playLoading.value = false;
    await getCurrentWindow().setFocus();
    await exit(0);
  } catch (e) {
    if (e instanceof Error || typeof e === 'string') {
      const errorString = e instanceof Error ? e.message : e;
      if (errorString.startsWith(`Timeout waiting for 'Finished.' in log`)) {
        const logData = await invoke<string[]>('get_log_data');
        errorStore.setError(e, {logData});
      } else {
        errorStore.setError(e);
      }
    } else {
      alert(e);
    }
    playLoading.value = false;
  }
}

async function handleExit() {
  invoke('kill_exit_1');
}

</script>