<template>
  <div class="d-flex ga-2">
    <v-select style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;" label="Render Distance (m)"
      :items="renderDistanceOptions" v-model="renderDistanceSelection" />
    <v-text-field type="number" :rules="[(v: number) => (v && v > 0) || 'required']"
      label="Distance (m)" v-model="renderSettings.renderDistance" />
  </div>
</template>

<script setup lang="ts">
import { useRenderSettingsStore } from "@/stores/renderSettings";
import { ref, watch } from "vue";

// const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
const { renderSettings } = useRenderSettingsStore();

const renderDistanceSelection = ref("Custom");
const renderDistanceOptions = ['200m (Near)', '300m (Normal)', '450m (Far)', 'Custom'];
updateBasedOnRenderDistance();

watch(() => renderDistanceSelection.value, (newValue) => {
  if (newValue === "Custom") {
  } else if (newValue === "200m (Near)") {
    renderSettings.renderDistance = 200;
  } else if (newValue === "300m (Normal)") {
    renderSettings.renderDistance = 300;
  } else if (newValue === "450m (Far)") {
    renderSettings.renderDistance = 450;
  }
});

watch(() => renderSettings.renderDistance, updateBasedOnRenderDistance);

function updateBasedOnRenderDistance() {
  const v = parseInt(renderSettings.renderDistance as unknown as string, 10);
  if (v === 200) {
    renderDistanceSelection.value = "200m (Near)";
  } else if (v === 300) {
    renderDistanceSelection.value = "300m (Normal)";
  } else if (v === 450) {
    renderDistanceSelection.value = "450m (Far)";
  } else {
    renderDistanceSelection.value = "Custom";
  }
}

</script>