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
    </v-form>
  </v-container>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getAsRdConfig } from '@/stores/renderSettings';
import Autostart from '@/components/Autostart.vue';
import { useTrainerSettingsStore } from '@/stores/trainerSettings';
import { exit } from '@tauri-apps/plugin-process';

const playLoading = ref(false);

async function handleAutoplay() {
  if (!playLoading.value) {
    await handlePlay();
  }
}

async function handlePlay() {
  playLoading.value = true;
  try {
    console.log (await invoke('read_rd_config'));
    console.log(await invoke('write_rd_config', { config: getAsRdConfig() }));
    const { trainerSettings } = useTrainerSettingsStore();
    if (trainerSettings.changeFov || trainerSettings.use4xFonts) {
      const r2 = await invoke('run_trainer', { trainerSettings });
      console.log(r2);
    }
    playLoading.value = false;
    await getCurrentWindow().setFocus();
    await exit(0);
  } catch (e) {
    playLoading.value = false;
    alert(e);
  }
}

async function handleExit() {
  invoke('kill_exit_1');
}

</script>