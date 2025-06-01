<template>
  <v-card>
    <v-card-title>Trainer Options</v-card-title>
    <v-card-text class="pb-2">
      <div class="d-flex ga-2">
        <v-checkbox
          v-model="trainerSettings.use4xFonts"
          label="Use 4x Fonts"
        />
        <div class="d-flex ga-1 align-center">
          <v-checkbox
            v-model="trainerSettings.disableDirectInput"
            label="Disable DirectInput"
          />
          <DisableDirectInputTooltip />
        </div>
      </div>
      <FovSelectorRow :form-is-loading="formIsLoading" />
      <div class="d-flex ga-1 align-center">
        <v-checkbox
          v-model="trainerSettings.enableLogging"
          label="Enable Logging"
        />
        <EnableLoggingTooltip />
        <v-checkbox
          v-model="trainerSettings.enableCustomControls"
          label="Custom Controls"
        />
      </div>
      <v-checkbox
        v-model="trainerSettings.makeGhostsOpaque"
        label="Make replay ghosts opaque"
      />
      <div class="d-flex ga-1 align-center">
        <v-checkbox
          v-model="trainerSettings.matchGhostSoundsToCharacter"
          label="Match ghost sounds to character"
        />
        <GhostSoundsTooltip />
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useTrainerUISettingsStore } from "@/stores/trainerSettings";
import EnableLoggingTooltip from "./EnableLoggingTooltip.vue";
import FovSelectorRow from "./FovSelectorRow.vue";
import GhostSoundsTooltip from "./GhostSoundsTooltip.vue";
const { trainerSettings } = useTrainerUISettingsStore();

const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
</script>
