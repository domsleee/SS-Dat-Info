<script setup lang="ts">
import HomePage from "./pages/HomePage.vue";
import { attachConsole } from "@tauri-apps/plugin-log";
import { commands } from "./bindings";
import { onMounted, nextTick } from 'vue';
import { timings } from "./services/timings";

onMounted(async () => {
  // crazy hack: https://github.com/tauri-apps/tauri/issues/1564
  void commands.showWindow();

  const onMounted1 = performance.now();
  await nextTick();
  const onMounted2 = performance.now();

  const startupInfo = {
    onMounted1,
    onMounted2,
    createAppTime: timings.getResult("createApp"),
    mountAppTime: timings.getResult("mountApp"),
    registerPluginsTime: timings.getResult("registerPlugins"),
    totalStartupTime: timings.getResult("totalStartup")
  };
  console.log("Startup info:", startupInfo);
  await commands.logStartupTime(startupInfo);

  await attachConsole();
});

</script>

<template>
  <main class="container">
    <HomePage />
  </main>
</template>

<style>
a {
  color: #6d8bef;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

</style>