<script setup lang="ts">
import HomePage from "./pages/HomePage.vue";
import { invoke } from "@tauri-apps/api/core";
import { attachConsole } from "@tauri-apps/plugin-log";

attachConsole();

// crazy hack: https://github.com/tauri-apps/tauri/issues/1564
window.addEventListener("DOMContentLoaded", () => {
  invoke("show_window");
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