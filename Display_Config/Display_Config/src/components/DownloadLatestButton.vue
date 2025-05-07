<template>
  <div v-if="hasUpdate">
    <v-tooltip :text="'Install ' + updateStatus.latestVersion" location="top" :visible="hasUpdate">
      <template v-slot:activator="{ props }">
        <v-btn 
          v-bind="props"
          icon="mdi-download" 
          variant="text" 
          color="green-darken-1" 
          size="large"
          @click="downloadLatest()"
        />
      </template>
    </v-tooltip>
  </div>
</template>

<script lang="ts" setup>
import { checkForUpdates, update } from '@/services/updaterService';
import { ref } from 'vue';

const hasUpdate = ref(false);
let updateStatus = {
  latestVersion: '0.0.0',
  currentVersion: '0.0.0',
};

checkForUpdates().then((x) => {
  updateStatus = x;
  hasUpdate.value = updateStatus.latestVersion !== updateStatus.currentVersion;
});

function downloadLatest() {
  update(updateStatus.latestVersion);
}


</script>