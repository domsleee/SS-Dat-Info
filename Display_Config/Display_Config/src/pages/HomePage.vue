<template>
  <v-container style="max-width: 400px">
    <v-form ref="form" :disabled="playLoading">
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

<style>
.v-field {
  --v-field-padding-start: 12px;
  --v-field-padding-end: 12px;
}
.v-field.v-field--appended {
  --v-field-padding-end: 0px;
}
</style>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { handlePlayAsync } from '../services/handlePlay';
import Autostart from '@/components/Autostart.vue';
import { VForm } from 'vuetify/components';

const playLoading = ref(false);
const form = ref<VForm | undefined>();
async function handleAutoplay() {
  if (!playLoading.value) {
    await handlePlay();
  }
}

async function handlePlay() {
  const { valid } = await form.value!.validate();
  if (!valid) return;
  // playLoading.value = true; await sleep(3000); playLoading.value = false; return;
  await handlePlayAsync(playLoading);
}

async function handleExit() {
  invoke('kill_exit_1');
}

</script>