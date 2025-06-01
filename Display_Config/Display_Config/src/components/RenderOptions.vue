<template>
  <v-card>
    <v-card-title>Graphics Options</v-card-title>
    <v-card-text class="pb-2">
      <div class="d-flex ga-2 flex-column">
        <div class="d-flex ga-2">
          <v-select
            v-model="renderSettings.renderer"
            label="Renderer"
            style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;"
            :items="renderDevices"
          />
          <v-select
            v-model="renderSettings.cardId as unknown as string"
            label="Card ID"
            :disabled="formIsLoading || getCardIds(renderSettings.renderer).isLoading"
            :items="getCardIds(renderSettings.renderer).value"
          />
        </div>
        <div class="d-flex ga-2">
          <v-combobox
            v-model="renderSettings.resolution"
            style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;"
            label="Resolution"
            :disabled="formIsLoading || getResolutions(renderSettings.renderer).isLoading"
            :items="getResolutions(renderSettings.renderer).value"
            :rules="[(v: string) => validateResolution(v)]"
          />
          <v-select
            v-model="renderSettings.colourDepth"
            label="Depth"
            :items="colorDepths"
          />
        </div>
        <div class="d-flex ga-2">
          <LanguageSelect style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;" />
          <v-checkbox
            v-model="renderSettings.fullscreen"
            label="Full screen"
          />
        </div>
        <RenderDistanceRow :form-is-loading="formIsLoading" />
        <div class="d-flex ga-2">
          <v-select
            v-model="renderSettings.groundDetail"
            label="Ground Detail"
            :items="groundDetailOptions"
            item-title="text"
            item-value="value"
            style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;"
          />
          <v-select
            v-model="renderSettings.ghostPlayer"
            label="Ghost Player"
            :items="ghostPlayers"
          />
        </div>
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

const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
const { renderSettings } = useRenderSettingsStore();

const ghostPlayers = ["Vincent", "Keith", "Ulrika", "Akiko", "Mike", "Karl", "Guide"];
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

const groundDetailOptions = [
  { text: '1 (Very Low)', value: 1 },
  { text: '2 (Low)', value: 2 },
  { text: '3 (Normal)', value: 3 },
  { text: '4 (Maximum)', value: 4 }
];

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

function validateResolution(resolution: string | undefined) {
  if (!resolution) {
    return 'required';
  }
  const spl = resolution.split("x");
  if (spl.length !== 2) {
    return 'invalid format';
  }
  const [width, height] = resolution.split("x").map(Number);
  return !isNaN(width) && !isNaN(height) && width > 0 && height > 0;
}

</script>
