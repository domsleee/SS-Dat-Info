<template>
  <v-card>
    <v-card-title>Graphics Options</v-card-title>
    <v-card-text>
      <div class="d-flex ga-2 flex-column">
        <v-select
          v-model="renderSettings.renderer"
          label="Renderer"
          :items="renderDevices"
        />
        <v-select
          v-model="renderSettings.cardId"
          label="Card ID"
          :disabled="formIsLoading || getCardIds(renderSettings.renderer).isLoading"
          :items="getCardIds(renderSettings.renderer).value"
        />
        <div class="d-flex ga-2">
          <v-combobox
            v-model="renderSettings.resolution"
            style="min-width: 60%"
            label="Resolution"
            :disabled="formIsLoading || getResolutions(renderSettings.renderer).isLoading"
            :items="getResolutions(renderSettings.renderer).value"
          />
          <v-select
            v-model="renderSettings.colourDepth"
            label="Depth"
            :items="colorDepths"
          />
        </div>
        <v-checkbox v-model="renderSettings.fullscreen" label="Full screen"></v-checkbox>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { getSyncPromise, type SyncPromise } from "../services/syncPromise";
import { useRenderSettingsStore } from "@/stores/renderSettings";
import {
  fetchResolutions,
  fetchCardIds,
} from "../services/resolutionAndCardIdFetcher";

// fixme: it should just use v-form disabled...
const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
const { renderSettings } = useRenderSettingsStore();

const renderDevices = ["DirectX6", "DirectX7", "OpenGL", "Software2"];
const colorDepths = ["16bit", "32bit"];

const resolutionData: Record<string, SyncPromise<string[]>> = {};
function getResolutions(renderer?: string) {
  if (!renderer) {
    return { value: [], isLoading: true };
  }
  if (!(renderer in resolutionData)) {
    resolutionData[renderer] = getSyncPromise(fetchResolutions(renderer));
  }
  return resolutionData[renderer];
}

const cardIdData: Record<string, SyncPromise<string[]>> = {};
function getCardIds(renderer?: string) {
  if (!renderer) {
    return { value: [], isLoading: true };
  }
  if (!(renderer in cardIdData)) {
    cardIdData[renderer] = getSyncPromise(fetchCardIds(renderer));
  }
  return cardIdData[renderer];
}
</script>
