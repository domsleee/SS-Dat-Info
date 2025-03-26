<template>
  <div class="d-flex align-center ga-2">
    <v-checkbox v-model="settings.autostart" :label="getLabel()" />
    <v-checkbox v-model="settings.autoCloseOthers" label="Auto close others" />
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { useAutostartStore } from '../stores/autostartStore';
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["auto-play"]);
const { settings } = useAutostartStore();
const isRunning = ref(false);
const timeLeft = ref(1500);
let interval: NodeJS.Timeout | undefined = undefined;

if (settings.autostart) {
  isRunning.value = true;
  interval = setInterval(() => {
    timeLeft.value -= 50;
    if (timeLeft.value <= 0) {
      clearInterval(interval);
      timeLeft.value = 0;
      emit("auto-play");
    }
  }, 50);
}

if (settings.autoCloseOthers) {
  invoke("close_others");
}

watch(() => settings.autostart, (newValue) => {
  if (!newValue) {
    isRunning.value = false;
    if (interval) {
      clearInterval(interval);
      interval = undefined;
    }
  }
});

function getLabel() {
  if (settings.autostart) {
    if (isRunning.value) {
      return "Auto start: " + formatTime(timeLeft.value);
    }
    return "Auto start (next open)";
  }
  return "Auto start";
}

function formatTime(time: number) {
  const seconds = Math.floor(time / 1000);
  const tenths = Math.floor((time % 1000) / 100).toFixed(0);
  return `${seconds}.${tenths}`;
}

</script>
