<template>
  <v-tooltip
    text="The default ghost sounds are mike. Enabling this will make them use speech_cfg.txt for the ghost player (first entry of player_types.txt) or guide."
    location="bottom"
  >
    <template #activator="{ props }">
      <v-icon
        :icon="mdiInformation"
        class="text-medium-emphasis"
        v-bind="props"
        @click="openPlayerTypes"
      />
    </template>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { commands } from '@/bindings';
import { mdiInformation } from '@mdi/js';

async function openPlayerTypes() {
  await commands.openPlayerTypes();
}
</script>
