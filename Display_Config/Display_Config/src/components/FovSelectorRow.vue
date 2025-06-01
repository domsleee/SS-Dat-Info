<template>
  <div class="d-flex align-items-center ga-1">
    <v-select
      v-model="trainerSettings.fovType"
      label="FOV Option"
      :items="fovTypes"
      style="flex-basis: 125px; flex-shrink: 0"
    />
    <v-text-field
      v-model="trainerSettings.fovWidth"
      type="number"
      :disabled="formIsLoading || trainerSettings.fovType !== 'Custom'"
      :rules="[(v: number) => validateNumber(v)]"
      label="Width"
      density="compact"
    />
    <v-text-field
      v-model="trainerSettings.fovHeight"
      type="number"
      :disabled="formIsLoading || trainerSettings.fovType !== 'Custom'"
      :rules="[(v: number) => validateNumber(v)]"
      label="Height"
    />
    <div
      class="align-self-center ml-1"
      style="flex-basis: 37px; text-align: center; flex-shrink: 0"
    >
      <p>{{ getFovFactor() }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">

import { useTrainerUISettingsStore } from "@/stores/trainerSettings";
import type { FovType } from "../services/fovOptions";
import { watch } from "vue";
import { useRenderSettingsStore } from "@/stores/renderSettings";
const { trainerSettings } = useTrainerUISettingsStore();
const { renderSettings } = useRenderSettingsStore();
const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();

const fovTypes: Array<FovType> = [
  "MatchRes",
  "Original",
  "Custom",
];

fixBasedOnResolution(renderSettings.resolution);
watch(() => trainerSettings.fovType, (newValue) => {
  fixBasedOnResolution(renderSettings.resolution);
  if (newValue === "Original") {
    trainerSettings.fovWidth = 800;
    trainerSettings.fovHeight = 600;
  }
});

watch(() => renderSettings.resolution, (newValue) => fixBasedOnResolution(newValue));

function fixBasedOnResolution(resolution: string | undefined) {
  if (trainerSettings.fovType !== "MatchRes") return;
  if (!resolution) return;
  const spl = resolution.split("x");
  if (spl.length !== 2) return;
  const [width, height] = spl.map(Number);
  trainerSettings.fovWidth = width;
  trainerSettings.fovHeight = height;
}

function getFovFactor() {
  if (!trainerSettings.fovWidth || !trainerSettings.fovHeight) {
    return undefined;
  }
  return (
    trainerSettings.fovWidth /
    trainerSettings.fovHeight /
    (4 / 3)
  ).toFixed(2);
}

function validateNumber(v: number | undefined) {
  if (v === undefined) {
    return "required";
  }
  if (isNaN(v)) {
    return "invalid number";
  }
  if (v <= 0) {
    return "must be greater than 0";
  }
  return true;
}

</script>
