<template>
  <div>
    <v-dialog persistent v-model="dialog" max-width="600px" min-height="240px" class="UpdateDialog">
      <v-card v-if="state.key === StateKey.Checking" prepend-icon="mdi-update" title="Updater">
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-progress-circular
              indeterminate
              color="primary"
              size="36"
              class="mr-4"
            ></v-progress-circular>
            <div>
              <div class="text-h6 font-weight-medium">Checking for updates...</div>
              <div class="text-body-2 text-medium-emphasis">Please wait while we check for the latest version.</div>
            </div>
          </div>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            class="ms-auto"
            text="Cancel"
            @click="closeDialog()"
          ></v-btn>
        </template>
      </v-card>

      <v-card v-if="state.key === StateKey.UpdateAvailable" prepend-icon="mdi-update" title="Updater" style="height: 500px">
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon icon="mdi-download" size="36" class="mr-4" />
            <div>
              <div class="text-h6 font-weight-medium">Update Available!</div>
              <div class="text-body-2 text-medium-emphasis">Update from {{ state.currentVersion }} --> {{ state.latestVersion }}?</div>
            </div>
          </div>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            text="Install"
            @click="doDownload()"
          ></v-btn>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          ></v-btn>
        </template>
      </v-card>

      <v-card v-if="state.key === StateKey.Downloading" prepend-icon="mdi-update" title="Updater">
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-progress-circular
              indeterminate
              color="primary"
              size="36"
              class="mr-4"
            />
            <div class="flex-grow-1">
              <div class="text-h6 font-weight-medium">Installing</div>
              <div class="text-body-2 text-medium-emphasis">Downloading {{ state.latestVersion }} from <a href="https://github.com/domsleee/SS-Dat-Info/releases/latest" target="_blank">github</a></div>
              <v-progress-linear
                v-model="state.progress"
                height="25"
                class="mt-2"
                color="primary"
              >
                <strong>{{ Math.floor(state.progress) }}%</strong>
              </v-progress-linear>
            </div>
          </div>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          ></v-btn>
        </template>
      </v-card>

      <v-card v-if="state.key === StateKey.NoUpdateAvailable" prepend-icon="mdi-update" title="Updater">
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon icon="mdi-check" size="36" class="mr-4" />
            <div>
              <div class="text-h6 font-weight-medium">Latest Version</div>
              <div class="text-body-2 text-medium-emphasis">{{ state.latestVersion }} is the latest version.</div>
            </div>
          </div>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          ></v-btn>
        </template>
      </v-card>

      <v-card v-if="state.key === StateKey.Finished" prepend-icon="mdi-update" title="Updater">
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon icon="mdi-check" size="36" class="mr-4" />
            <div>
              <div class="text-h6 font-weight-medium">Finished</div>
              <div class="text-body-2 text-medium-emphasis">Installed {{ state.latestVersion }}. Restart the program to see the change.</div>
            </div>
          </div>
        </v-card-text>

        <template v-slot:actions>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          ></v-btn>
        </template>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { checkForUpdates } from '@/services/updaterService';
import { Channel, invoke } from '@tauri-apps/api/core';
import { runWithErrorHandler } from '@/stores/errorStore';

type State = StateChecking | StateDownloadAvailable | StateDownloading | StateFinished | StateNoUpdateAvailable;
enum StateKey {
  Checking,
  NoUpdateAvailable,
  UpdateAvailable,
  Downloading,
  Finished,
}
const dialog = ref(false);
const state = ref<State>({ key: StateKey.Checking });

async function openDialog() {
  dialog.value = true;
  state.value = { key: StateKey.Checking };
  const checkForUpdatesResult = await checkForUpdates();
  if (checkForUpdatesResult.currentVersion === checkForUpdatesResult.latestVersion) {
    state.value = {
      key: StateKey.NoUpdateAvailable,
      currentVersion: checkForUpdatesResult.currentVersion,
      latestVersion: checkForUpdatesResult.latestVersion,
    };
    return;
  }

  state.value = {
    key: StateKey.UpdateAvailable,
    currentVersion: checkForUpdatesResult.currentVersion,
    latestVersion: checkForUpdatesResult.latestVersion,
  };
}

async function doDownload() {
  await runWithErrorHandler(async () => {
    if (state.value.key !== StateKey.UpdateAvailable) return;
    state.value = {
      key: StateKey.Downloading,
      latestVersion: state.value.latestVersion,
      progress: 0,
    }
    const url = `https://github.com/domsleee/SS-Dat-Info/releases/download/${state.value.latestVersion}/Display_Config_${state.value.latestVersion}.zip`;

    await invoke('download_and_extract', {
      url,
      onEvent: new Channel<DownloadEvent>((message) => {
        if (message.event === 'downloadProgress' && state.value.key === StateKey.Downloading) {
          const { progressTotal, total } = message.data;
          state.value.progress = parseFloat(((progressTotal / total) * 100).toFixed(0));
        }
      })
    });

    state.value = {
      key: StateKey.Finished,
      latestVersion: state.value.latestVersion,
    };
  });
}

interface StateChecking {
  key: StateKey.Checking;
}

interface StateDownloadAvailable {
  key: StateKey.UpdateAvailable;
  currentVersion: string;
  latestVersion: string;
}

interface StateNoUpdateAvailable {
  key: StateKey.NoUpdateAvailable;
  currentVersion: string;
  latestVersion: string;
}

interface StateDownloading {
  key: StateKey.Downloading;
  latestVersion: string;
  progress: number;
}

interface StateFinished {
  key: StateKey.Finished;
  latestVersion: string;
}

type DownloadEvent =
  {
    event: 'downloadProgress';
    data: {
      progress: number;
      progressTotal: number;
      total: number;
      transfer_speed: number;
    };
  };


function closeDialog() {
  dialog.value = false;
}

defineExpose({ openDialog });
</script>
