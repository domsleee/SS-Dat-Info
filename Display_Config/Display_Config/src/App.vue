<script setup lang="ts">
import HomePage from "./pages/HomePage.vue";
import { attachConsole } from "@tauri-apps/plugin-log";
import { commands } from "./bindings";
void attachConsole();

// crazy hack: https://github.com/tauri-apps/tauri/issues/1564
window.addEventListener("DOMContentLoaded", async () => {
  await commands.showWindow();
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