<template>
  <v-card>
    <v-card-title>Trainer Options</v-card-title>
    <v-card-text>
      <v-checkbox v-model="trainerSettings.use4xFonts" label="Use 4x Fonts" />
      <div class="d-flex align-items-center ga-2">
        <v-checkbox
          class="align-self-center"
          v-model="trainerSettings.changeFov"
          label="Change FOV"
        />
        <v-text-field
          class="fovWidth"
          v-model="trainerSettings.fovWidth"
          :disabled="formIsLoading || !trainerSettings.changeFov"
          label="Width"
        />
        <v-text-field
          class="fovHeight"
          v-model="trainerSettings.fovHeight"
          :disabled="formIsLoading || !trainerSettings.changeFov"
          hide-details
          label="Height"
        />
        <p class="align-self-center">{{ getFovFactor() }}</p>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useTrainerSettingsStore } from "@/stores/trainerSettings";
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
  ).toFixed(3);
}
</script>

<style scoped>
.fovWidth,
.fovHeight {
  max-width: 5em;
}
</style>
