<template>
  <div v-if="hasUpdate">
    <v-tooltip
      :text="'Install ' + updateStatus.latestVersion"
      location="top"
      :visible="hasUpdate"
    >
      <template #activator="{ props }">
        <v-btn 
          v-bind="props"
          :icon="mdiDownload" 
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
import { onMounted, ref } from 'vue';
import { mdiDownload } from '@mdi/js';

const hasUpdate = ref(false);
let updateStatus = {
  latestVersion: '0.0.0',
  currentVersion: '0.0.0',
};

async function run() {
  const x = await checkForUpdates();
  updateStatus = x;
  hasUpdate.value = updateStatus.latestVersion !== updateStatus.currentVersion;
}

async function downloadLatest() {
  await update(updateStatus.latestVersion);
}

onMounted(async () => {
  await run();
});

</script>