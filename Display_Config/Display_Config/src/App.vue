<script setup lang="ts">
import HomePage from "./pages/HomePage.vue";
import { attachConsole } from "@tauri-apps/plugin-log";
import { commands } from "./bindings";
import { onMounted, nextTick } from 'vue';
void attachConsole();

// crazy hack: https://github.com/tauri-apps/tauri/issues/1564
window.addEventListener("DOMContentLoaded", async () => {
  await commands.showWindow();
});

onMounted(async () => {
  const onMounted1 = performance.now();
  console.log("onMounted(1)", onMounted1);
  await nextTick();
  const onMounted2 = performance.now();
  console.log("onMounted(2)", onMounted2);
  await commands.logStartupTime({ onMounted1, onMounted2 });
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