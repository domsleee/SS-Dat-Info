<template>
  <div>
    <v-menu
      v-model="menu"
      :close-on-content-click="false"
      min-width="320"
      rounded="lg"
      offset="5"
      :z-index="100"
    >
      <template #activator="{ props }">
        <v-btn
          v-bind="props"
          :icon="mdiCog"
          size="large"
          class="settings-button"
        />
      </template>
      
      <v-card
        elevation="16"
        rounded="lg"
        class="menu-card"
      >
        <v-divider />
        
        <v-list
          density="compact"
          bg-color="background"
        >
          <v-list-item
            v-for="(item, i) in items"
            :key="i"
            :title="item.title"
            :prepend-icon="item.icon"
            :ripple="true"
            link
            class="menu-item my-1"
            @click="handleAction(item.action)"
          />
        </v-list>
      </v-card>
    </v-menu>

    <AboutDialog ref="aboutDialog" />
    <UpdateDialog ref="updateDialog" />
  </div>
</template>

<script setup>
import { mdiCog, mdiInformationOutline, mdiUpdate } from '@mdi/js';
import { ref } from 'vue';
const aboutDialog = ref(null);
const updateDialog = ref(null);

const menu = ref(false);
const items = ref([
  { title: 'Check for updates', icon: mdiUpdate, action: 'checkUpdates' },
  { title: 'About', icon: mdiInformationOutline, action: 'about' },
]);

const handleAction = (action) => {
  if (action === 'checkUpdates') {
    updateDialog.value.openDialog();
  } else if (action === 'about') {
    aboutDialog.value.openDialog();
  }
  menu.value = false;
};
</script>

<style scoped>
.settings-button {
  transition: transform 0.2s ease;
}

.settings-button:hover {
  transform: rotate(30deg);
}
</style>