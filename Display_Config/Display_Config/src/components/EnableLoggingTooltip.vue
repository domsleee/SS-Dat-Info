<template>
  <v-tooltip
    text="The game will create more logs in Supreme/HMGEHLOG.TXT"
    location="bottom"
  >
    <template v-slot:activator="{ props }">
      <v-icon
        @click="openLogFile"
        icon="mdi-information"
        class="text-medium-emphasis"
        v-bind="props"
      ></v-icon>
    </template>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';


function openLogFile() {
  invoke('open_log_file');
}
</script>
