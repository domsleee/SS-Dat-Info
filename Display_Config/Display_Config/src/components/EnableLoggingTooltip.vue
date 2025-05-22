<template>
  <v-tooltip
    text="The game will create more logs in Supreme/HMGEHLOG.TXT, same as ori cheat"
    location="bottom"
  >
    <template v-slot:activator="{ props }">
      <v-icon
        @click="openLogFile"
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

function openLogFile() {
  commands.openLogFile();
}
</script>
