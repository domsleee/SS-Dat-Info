<template>
  <v-tooltip
    text="The game will create more logs in Supreme/HMGEHLOG.TXT, same as ori cheat"
    location="bottom"
  >
    <template #activator="{ props }">
      <v-icon
        :icon="mdiInformation"
        class="text-medium-emphasis"
        v-bind="props"
        @click="openLogFile"
      />
    </template>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { commands } from '@/bindings';
import { mdiInformation } from '@mdi/js';

async function openLogFile() {
  await commands.openLogFile();
}
</script>
