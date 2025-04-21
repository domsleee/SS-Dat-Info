<template>
  <v-tooltip
    text="Fixes some start-up crashes, but disables joystick support."
    location="bottom"
  >
    <template v-slot:activator="{ props }">
      <v-icon
        icon="mdi-information"
        class="text-medium-emphasis"
        v-bind="props"
      ></v-icon>
    </template>
  </v-tooltip>
</template>
