<template>
  <v-select
    v-model="country"
    :items="countries"
    label="Language"
    item-title="name"
    item-value="code"
    return-object
  >
    <template #selection="{ item }">
      <div class="d-flex align-center">
        <span class="em mr-2">{{ getFlagEmoji(item.raw.flag) }}</span>
        {{ item.raw.name }}
      </div>
    </template>
    <template #item="{ item, props }">
      <v-list-item v-bind="props">
        <template #prepend>
          <span class="em mr-2">{{ getFlagEmoji(item.raw.flag) }}</span>
        </template>
        <template #title>
          {{ item.raw.name }}
        </template>
      </v-list-item>
    </template>
  </v-select>
</template>

<script setup lang="ts">
import { computed } from "vue";
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
    renderSettings.language = value!.name;
  }
});

function getFlagEmoji(countryCode: string): string {
  return String.fromCodePoint(...[...countryCode.toUpperCase()].map(
    char => 127397 + char.charCodeAt(0)
  ));
}

</script>

<style>
.em {
  font-size: 1.2em;
  line-height: 1em;
}
</style>
