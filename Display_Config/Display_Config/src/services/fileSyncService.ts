import { useRenderSettingsStore } from "@/stores/renderSettings";
import { invoke } from "@tauri-apps/api/core";
import { tryParseInt } from "./stringUtil";
import type { VForm } from "vuetify/components";
import { type Ref } from "vue";
import { writeDetailConfig } from "./handlePlay";
import { commands } from "@/bindings";


export async function loadFromFiles() {
  const { renderSettings } = useRenderSettingsStore();

  const rows = await invoke<Array<[string, string]>>('read_detail_config') ?? [];
  for (const [key, v] of rows) {
    if (key === 'visibility_cube_width' && tryParseInt(v)) {
      renderSettings.renderDistance = parseInt(v);
    }
    else if (key === 'ground_detail' && tryParseInt(v)) {
      renderSettings.groundDetail = parseInt(v);
    }
  }

  try {
    const playerType = await invoke<string>('get_first_player_type');
    renderSettings.ghostPlayer = playerType.charAt(0).toUpperCase() + playerType.slice(1);;

  } catch (e) {
    console.log(e);
  }
}

export async function setupFileSync(form: Ref<VForm | undefined>) {
  const renderSettingsStore  = useRenderSettingsStore();
  renderSettingsStore.$subscribe(async () => {
    const { valid } = await form.value!.validate();
    if (!valid) return;

    await writeDetailConfig();
    await commands.setFirstPlayerType(renderSettingsStore.renderSettings.ghostPlayer);
  });
}
