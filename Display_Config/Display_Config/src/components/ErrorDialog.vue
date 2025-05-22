<template>
  <v-dialog v-model="store.show" max-width="340" persistent>
    <v-card>
      <v-card-title class="text-h6 bg-error text-white">
        <v-icon start :icon="mdiAlertCircle" class="mr-2"></v-icon>
        Error
      </v-card-title>

      <v-card-text class="pt-4">
        <span>{{ store.error }}</span>
        <pre v-if="store.options?.logData !== undefined" class="mt-1">{{ store.options.logData.join('\n') }}</pre>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="error" variant="text" @click="handleClose"> Close </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useErrorStore } from "@/stores/errorStore";
import { mdiAlertCircle } from "@mdi/js";

const store = useErrorStore();

const handleClose = () => {
  store.clearError();
};
</script>

<style>
pre {
    border: 1px solid black;
    overflow-y: auto;
    overflow-x: auto;
    max-height: 200px;
    padding: 3px;
}
</style>