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
        (v: number) => v <= maxSafeDistance || `max ${maxSafeDistance}m at Ground Detail ${renderSettings.groundDetail} — the terrain batch runs out of 16-bit vertex indices past that and distant triangles shred; lower Ground Detail to go further`,
      ]"
      :hint="`max ${maxSafeDistance}m at Ground Detail ${renderSettings.groundDetail}`"
      persistent-hint
      label="Distance (m)"
    />
  </div>
</template>

<script setup lang="ts">
import { useRenderSettingsStore } from "@/stores/renderSettings";
import { computed, ref, watch } from "vue";

// const { formIsLoading } = defineProps<{ formIsLoading: boolean }>();
const { renderSettings } = useRenderSettingsStore();

const renderDistanceSelection = ref("Custom");
const renderDistanceOptions = ['200m (Near)', '300m (Normal)', '450m (Far)', 'Custom'];

// The engine caps VISIBLE terrain at 65,536 vertices (a 16-bit-addressable
// vertex pool in the mesh build): past it, far patches draw shredded
// (measured live on Alpine Easy: detail 4 shreds between 480 and 500m —
// exactly where 17x17-vertex patches exhaust the index space; 600m at
// detail 3 is clean). Mirror of max_safe_render_distance in
// src-tauri/src/detail_config.rs, which enforces the same cap on write.
const maxSafeDistance = computed(() => {
  // Number(): the persisted store can hand this back as a string.
  switch (Number(renderSettings.groundDetail)) {
    case 4: return 480;
    case 3: return 600;
    case 2: return 900;
    default: return 1200;
  }
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