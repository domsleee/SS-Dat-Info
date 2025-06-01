<template>
  <v-container style="max-width: 400px">
    <v-form
      ref="form"
      :disabled="playLoading"
    >
      <RenderOptions :form-is-loading="playLoading" />
      <TrainerOptions
        :form-is-loading="playLoading"
        class="mt-2"
      />
      <Autostart @auto-play="handleAutoplay()" />
      <div class="mt-2 ga-2 d-flex justify-end">
        <DownloadLatestButton />
        <SettingsButton />
        <v-btn
          size="x-large"
          color="indigo-darken-3"
          :loading="playLoading"
          @click="handlePlay"
        >
          Play
        </v-btn>
        <v-btn
          size="x-large"
          color=""
          @click="handleExit"
        >
          Exit
        </v-btn>
      </div>
      <ErrorDialog />
    </v-form>
  </v-container>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { handlePlayAsync } from '../services/handlePlay';
import { VForm } from 'vuetify/components';
import { loadFromFiles, setupFileSync } from '../services/fileSyncService';

const playLoading = ref(false);
const form = ref<VForm | undefined>();
async function handleAutoplay() {
  if (!playLoading.value) {
    await handlePlay();
  }
}
void loadFromFiles();
void setupFileSync(form);

async function handlePlay() {
  const { valid } = await form.value!.validate();
  if (!valid) return;
  await handlePlayAsync(playLoading);
}

async function handleExit() {
  await invoke('kill_exit_1');
}

</script>

<style>
.v-field {
  --v-field-padding-start: 12px;
  --v-field-padding-end: 12px;
}
.v-field.v-field--appended {
  --v-field-padding-end: 0px;
}
</style>