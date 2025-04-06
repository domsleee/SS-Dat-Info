<template>
  <v-select
    v-model="country"
    :items="countries"
    label="Language"
    item-title="name"
    item-value="code"
    return-object
  >
    <template v-slot:selection="{ item }">
      <div class="d-flex align-center">
        <i :class="['mr-2', 'em', `em-flag-${item.raw.flag}`]"></i>
        {{ item.raw.name }}
      </div>
    </template>
    <template v-slot:item="{ item, props }">
      <v-list-item v-bind="props">
        <template v-slot:prepend>
          <i :class="['mr-2', 'em', `em-flag-${item.raw.flag}`]"></i>
        </template>
        <template v-slot:title>
          {{ item.raw.name }}
        </template>
      </v-list-item>
    </template>
  </v-select>
</template>

<script setup>
import { ref, computed } from "vue";
import { useRenderSettingsStore } from "@/stores/renderSettings";

const { renderSettings } = useRenderSettingsStore();

const countries = [
  { name: "English", code: "en", flag: "gb" },
  { name: "Finnish", code: "fi", flag: "fi" },
  // { name: "Italian", code: "it", flag: "it" },
  { name: "French", code: "fr", flag: "fr" },
  { name: "German", code: "de", flag: "de" },
];

const country = computed({
  get: () => {
    return countries.find(c => c.name === renderSettings.language) || countries[0];
  },
  set: (value) => {
    renderSettings.language = value.name;
  }
});

</script>

<style>
.em {
  width: 1.2em;
  height: 1.2em;
}
</style>
