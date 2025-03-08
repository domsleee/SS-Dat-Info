<template>
  <v-card>
    <v-card-title>Trainer Options</v-card-title>
    <v-card-text class="pb-2">
      <v-checkbox v-model="trainerSettings.use4xFonts" label="Use 4x Fonts" />
      <div class="d-flex align-items-center ga-2">
        <v-checkbox
          class="align-self-center flex-grow"
          v-model="trainerSettings.changeFov"
          label="Change FOV"
          style="min-width: 120px"
        />
        <v-text-field
          class="fovWidth"
          type="number"
          v-model="trainerSettings.fovWidth"
          :disabled="formIsLoading || !trainerSettings.changeFov"
          hide-spin-buttons
          label="Width"
        />
        <v-text-field
          class="fovHeight"
          type="number"
          v-model="trainerSettings.fovHeight"
          :disabled="formIsLoading || !trainerSettings.changeFov"
          hide-spin-buttons
          label="Height"
        />
        <p class="align-self-center">{{ getFovFactor() }}</p>
      </div>
      <div class="d-flex ga-2" style="align-items: center">
        <v-checkbox v-model="trainerSettings.enableLogging" label="Enable Logging" />
        <EnableLoggingTooltip />
      </div>
      <v-checkbox v-model="trainerSettings.makeGhostsOpaque" label="Make replay ghosts opaque" />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useTrainerSettingsStore } from "@/stores/trainerSettings";
import EnableLoggingTooltip from "./EnableLoggingTooltip.vue";
const { trainerSettings } = useTrainerSettingsStore();

// fixme: it should just use v-form disabled...
const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();

function getFovFactor() {
  if (!trainerSettings.fovWidth || !trainerSettings.fovHeight) {
    return "";
  }
  return (
    trainerSettings.fovWidth /
    trainerSettings.fovHeight /
    (4 / 3)
  ).toFixed(2);
}

</script>

<style scoped>
.fovWidth,
.fovHeight {
  max-width: 5em;
}
</style>
