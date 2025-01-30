<template>
  <v-select v-model="country" :items="countries" label="Language">
    <template v-slot:selection="{ props, item }">
      <v-list-item v-bind="props">
        <template v-slot="prepend">
          <i :class="['mr-2', 'em', 'em-flag-ad']"></i>
          aa
        </template>
      </v-list-item>
    </template>
    <template v-slot:item="{ props, item }">
      <v-list-item v-bind="props">
      </v-list-item>
    </template>
  </v-select>
</template>

<script lang="ts" setup>
import { ref } from "vue";
const country = ref("English");
const countries = ["English", "French"];
</script>
