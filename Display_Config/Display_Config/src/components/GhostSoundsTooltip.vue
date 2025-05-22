<template>
  <v-tooltip
    text="The default ghost sounds are mike. Enabling this will make them use speech_cfg.txt for the ghost player (first entry of player_types.txt) or guide."
    location="bottom"
  >
    <template v-slot:activator="{ props }">
      <v-icon
        @click="openPlayerTypes"
        :icon="mdiInformation"
        class="text-medium-emphasis"
        v-bind="props"
      ></v-icon>
    </template>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { commands } from '@/bindings';
import { mdiInformation } from '@mdi/js';

function openPlayerTypes() {
  commands.openPlayerTypes();
}
</script>
