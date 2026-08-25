<template>
  <div class="d-flex ga-2">
    <v-select
      v-model="renderDistanceSelection"
      style="flex-basis: 60%; flex-shrink: 0; flex-grow: 0;"
      label="Render Distance (m)"
      :items="renderDistanceOptions"
    />
    <v-text-field
      v-model="renderSettings.renderDistance"
      type="number"
      :rules="[
        (v: number) => (v && v > 0) || 'required',
        (v: number) => v <= maxSafeDistance || `max ${maxSafeDistance}m at Ground Detail ${renderSettings.groundDetail} — beyond this the game is unstable (Village Hard crashes at 800+) or, at Ground Detail 4 without the Extended render distance fix, distant triangles vanish past 480m`,
      ]"
      :hint="`max ${maxSafeDistance}m at Ground Detail ${renderSettings.groundDetail}`"
      persistent-hint
      label="Distance (m)"
    />
  </div>
</template>

<script setup lang="ts">
import { useRenderSettingsStore } from "@/stores/renderSettings";
import { useTrainerUISettingsStore } from "@/stores/trainerSettings";
import { computed, ref, watch } from "vue";

// const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
const { renderSettings } = useRenderSettingsStore();
const { trainerSettings } = useTrainerUISettingsStore();

const renderDistanceSelection = ref("Custom");
const renderDistanceOptions = ['200m (Near)', '300m (Normal)', '450m (Far)', 'Custom'];

// The ground renderer skips strips longer than its row cap (rows are 1.2m x
// detail step), so terrain past cap x spacing vanishes as missing triangles
// (measured live on Alpine Easy: detail 4 clean at 480, shredded at 500).
// The "Extended render distance" trainer fix patches the cap 400 -> 500,
// lifting detail 4 from 480m to 600m. Mirror of max_safe_render_distance in
// src-tauri/src/detail_config.rs (which assumes the default-on patch).
const maxSafeDistance = computed(() => {
  const patched = trainerSettings.extendRenderDistance;
  // 600 is the certified ceiling at every detail: Village Hard crashes at
  // 800/1200 mid-run (dense object map), while 600 is soak-tested clean.
  // Without the row-cap patch, detail 4 additionally shreds past 480.
  // Number(): the persisted store can hand this back as a string.
  if (Number(renderSettings.groundDetail) === 4 && !patched) return 480;
  return 600;
});
updateBasedOnRenderDistance();

watch(() => renderDistanceSelection.value, (newValue) => {
  if (newValue === "Custom") {
    // do nothing
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