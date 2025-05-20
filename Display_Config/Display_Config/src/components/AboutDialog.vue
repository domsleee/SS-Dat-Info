<template>
  <div>
    <v-dialog v-model="dialog" max-width="600px">
      <v-card :prepend-icon="mdiInformationOutline" title="About">
        <v-card-text>
          <h3 class="mb-3">Version {{ version }}</h3>

          <p>Developed by <a href="https://github.com/domsleee" target="_blank">domsleee</a> with feedback from the 
              <a href="https://discord.gg/q3s7bZM7Kt" target="_blank">USSHF discord community</a>.
          </p>
          
          <p class="mt-1"> See also <a href="https://github.com/domsleee/SS-Dat-Info/wiki/Display_Config" target="_blank">Display_Config documentation</a>.
          </p>

          <h3 class="mt-4 mb-2">Issues & Suggestions?</h3>
          <p>The <a href="https://discord.com/channels/1305842972126085141/1334759048125485087" target="_blank">Display_Config troubleshooting thread</a> is a good place to ask</p>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            class="ms-auto"
            text="Ok"
            @click="dialog = false"
          ></v-btn>
        </template>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup>
import { commands } from "@/bindings";
import { ref } from 'vue';
import { mdiInformationOutline } from "@mdi/js";

const dialog = ref(false);
const version = ref('');

const openDialog = async () => {
  dialog.value = true;
  version.value = await commands.getVersion();
};
defineExpose({ openDialog });
</script>
