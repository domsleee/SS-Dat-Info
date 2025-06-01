<template>
  <div>
    <v-dialog
      v-model="isOpen"
      persistent
      max-width="600px"
      min-height="240px"
      class="UpdateDialog"
    >
      <v-card
        v-if="state.state.key === 'checking'"
        :prepend-icon="mdiUpdate"
        title="Updater"
      >
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-progress-circular
              indeterminate
              color="primary"
              size="36"
              class="mr-4"
            />
            <div>
              <div class="text-h6 font-weight-medium">
                Checking for updates...
              </div>
              <div class="text-body-2 text-medium-emphasis">
                Please wait while we check for the latest version.
              </div>
            </div>
          </div>
        </v-card-text>

        <template #actions>
          <v-btn
            class="ms-auto"
            text="Cancel"
            @click="closeDialog()"
          />
        </template>
      </v-card>

      <v-card
        v-if="state.state.key === 'updateAvailable'"
        :prepend-icon="mdiUpdate"
        title="Updater"
        style="height: 500px"
      >
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon
              :icon="mdiDownload"
              size="36"
              class="mr-4"
            />
            <div>
              <div class="text-h6 font-weight-medium">
                Update Available!
              </div>
              <div class="text-body-2 text-medium-emphasis">
                Update from {{ state.state.currentVersion }} --> <a
                  target="_blank"
                  :href="getReleaseLink(state.state.latestVersion)"
                >{{ state.state.latestVersion }}</a>?
              </div>
            </div>
          </div>
        </v-card-text>

        <template #actions>
          <v-btn
            text="Install"
            @click="update(state.state.latestVersion)"
          />
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          />
        </template>
      </v-card>

      <v-card
        v-if="state.state.key === 'downloading'"
        :prepend-icon="mdiUpdate"
        title="Updater"
      >
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-progress-circular
              indeterminate
              color="primary"
              size="36"
              class="mr-4"
            />
            <div class="flex-grow-1">
              <div class="text-h6 font-weight-medium">
                Installing
              </div>
              <div class="text-body-2 text-medium-emphasis">
                Downloading {{ state.state.latestVersion }} from <a
                  href="https://github.com/domsleee/SS-Dat-Info/releases/latest"
                  target="_blank"
                >github</a>
              </div>
              <v-progress-linear
                v-model="state.state.progress"
                height="25"
                class="mt-2"
                color="primary"
              >
                <strong>{{ Math.floor(state.state.progress) }}%</strong>
              </v-progress-linear>
            </div>
          </div>
        </v-card-text>

        <template #actions>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          />
        </template>
      </v-card>

      <v-card
        v-if="state.state.key === 'noUpdateAvailable'"
        :prepend-icon="mdiUpdate"
        title="Updater"
      >
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon
              :icon="mdiCheck"
              size="36"
              class="mr-4"
            />
            <div>
              <div class="text-h6 font-weight-medium">
                Latest Version
              </div>
              <div class="text-body-2 text-medium-emphasis">
                {{ state.state.latestVersion }} is the latest version.
              </div>
            </div>
          </div>
        </v-card-text>

        <template #actions>
          <v-btn
            text="Cancel"
            @click="closeDialog()"
          />
        </template>
      </v-card>

      <v-card
        v-if="state.state.key === 'finished'"
        :prepend-icon="mdiUpdate"
        title="Updater"
      >
        <v-card-text>
          <div class="d-flex align-center mb-3">
            <v-icon
              :icon="mdiCheck"
              size="36"
              class="mr-4"
            />
            <div>
              <div class="text-h6 font-weight-medium">
                Finished
              </div>
              <div class="text-body-2 text-medium-emphasis">
                Installed {{ state.state.latestVersion }}. Restart the program to see the change.
              </div>
            </div>
          </div>
        </v-card-text>

        <template #actions>
          <v-btn
            text="Close"
            @click="closeDialog()"
          />
        </template>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { checkForUpdates } from '@/services/updaterService';
import { update } from '@/services/updaterService';
import { runWithErrorHandler } from '@/stores/errorStore';
import { useUpdateDialogStore } from '@/stores/updateDialogStore';
import { commands } from '@/bindings';
import { mdiCheck, mdiDownload, mdiUpdate } from '@mdi/js'

const state = useUpdateDialogStore();
const isOpen = computed(() => state.state.key !== 'closed')

async function openDialog() {
  await runWithErrorHandler(async () => {
    state.state = { key: 'checking' };
    const checkForUpdatesResult = await checkForUpdates();
    if (checkForUpdatesResult.currentVersion === checkForUpdatesResult.latestVersion) {
      state.state = {
        key: 'noUpdateAvailable',
        currentVersion: checkForUpdatesResult.currentVersion,
        latestVersion: checkForUpdatesResult.latestVersion,
      };
      return;
    }
    
    state.state = {
      key: 'updateAvailable',
      currentVersion: checkForUpdatesResult.currentVersion,
      latestVersion: checkForUpdatesResult.latestVersion,
    };
  });
}

function getReleaseLink(version: string) {
  return `https://github.com/domsleee/SS-Dat-Info/releases/${version}`;
}


async function closeDialog() {
  if (state.state.key === 'downloading' && state.state.token) {
    await commands.cancelDownload(state.state.token);
  }
  state.state.key = 'closed';
}

defineExpose({ openDialog });
</script>
