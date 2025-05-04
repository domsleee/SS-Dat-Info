<template>
  <div>
    <!-- <v-btn @click="openLoadingDialog" color="green-lighten-2" icon="mdi-download" variant="text" size="large"></v-btn>
    
    <v-dialog v-model="loading" persistent max-width="350">
      <v-card rounded="lg" elevation="8">
        <v-card-text class="pt-6">
          <div class="d-flex align-center mb-3">
            <v-progress-circular
              indeterminate
              color="primary"
              size="36"
              class="mr-4"
            ></v-progress-circular>
            <div>
              <div class="text-h6 font-weight-medium">Downloading</div>
              <div class="text-body-2 text-medium-emphasis">Please wait while we download the latest version</div>
            </div>
          </div>
          
          <v-progress-linear
            v-if="showProgress"
            v-model="progressValue"
            color="primary"
            height="6"
            rounded
            class="mt-2"
          ></v-progress-linear>
        </v-card-text>
        
        <v-card-actions class="pt-0 pb-4 px-4">
          <v-spacer></v-spacer>
          <v-btn
            color="error"
            variant="text"
            @click="cancelLoading"
            class="text-none"
          >
            Cancel
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog> -->
  </div>
</template>

<script lang="ts" setup>
// import { ref } from 'vue';

// const loading = ref(false);
// const downloadTimer = ref(null);
// const progressValue = ref(0);
// const showProgress = ref(true);
// const totalTime = 9000;
// const updateInterval = 100;
// let progressInterval = null;

// const openLoadingDialog = () => {
//   progressValue.value = 0;
//   loading.value = true;
  
//   progressInterval = setInterval(() => {
//     if (progressValue.value < 100) {
//       const increment = Math.random() * 3 + 0.5;
//       progressValue.value = Math.min(99, progressValue.value + increment);
//     }
//   }, updateInterval);
  
//   downloadTimer.value = setTimeout(() => {
//     clearInterval(progressInterval);
//     progressValue.value = 100;
    
//     setTimeout(() => {
//       loading.value = false;
//       downloadTimer.value = null;
//     }, 500);
//   }, totalTime);
// };

// const cancelLoading = () => {
//   if (downloadTimer.value) {
//     clearTimeout(downloadTimer.value);
//     downloadTimer.value = null;
//   }
  
//   if (progressInterval) {
//     clearInterval(progressInterval);
//     progressInterval = null;
//   }
  
//   loading.value = false;
// };
</script>