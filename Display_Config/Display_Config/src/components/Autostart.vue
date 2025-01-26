<template>
  <v-checkbox v-model="autostart.autostart" :label="getLabel()" />
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { useAutostartStore } from '../stores/autostartStore';

const emit = defineEmits(["auto-play"]);
const autostart = useAutostartStore();
const isRunning = ref(false);
const timeLeft = ref(2500);
let interval: NodeJS.Timeout | undefined = undefined;

if (autostart.autostart) {
  isRunning.value = true;
  interval = setInterval(() => {
    console.log("HEY")
    timeLeft.value -= 50;
    if (timeLeft.value <= 0) {
      clearInterval(interval);
      timeLeft.value = 0;
      emit("auto-play");
    }
  }, 50);
}

watch(() => autostart.autostart, (newValue) => {
  if (!newValue) {
    isRunning.value = false;
    if (interval) {
      clearInterval(interval);
      interval = undefined;
    }
  }
});

function getLabel() {
  if (autostart.autostart) {
    if (isRunning.value) {
      return "auto start: " + formatTime(timeLeft.value);
    }
    return "auto start: will run on restart";
  }
  return "auto start";;
}

function formatTime(time: number) {
  const seconds = Math.floor(time / 1000);
  const tenths = Math.floor((time % 1000) / 100).toFixed(0);
  return `${seconds}.${tenths}`;
}

</script>
