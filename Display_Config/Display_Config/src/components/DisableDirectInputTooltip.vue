<template>
  <v-tooltip
    text="Fixes some start-up crashes, but disables joystick support."
    location="bottom"
  >
    <template #activator="{ props }">
      <v-icon
        :icon="mdiInformation"
        class="text-medium-emphasis"
        v-bind="props"
      />
    </template>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { mdiInformation } from '@mdi/js';
</script>
