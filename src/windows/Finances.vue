<script setup lang="ts">
import Overview from "./Finances/Overview.vue";
import Budgets from "./Finances/Budgets.vue";
import Rules from "./Finances/Rules.vue";
import TabbedWindow from "../components/windows/TabbedWindow.vue";
</script>

<template>
  <TabbedWindow title="Finance" :pos="{ x: 100, y: 50 }" :size="{ width: 800, height: 600 }">
    <template v-slot:Overview>
      <Overview />
    </template>
    <template v-slot:Budgets>
      <Budgets />
    </template>
    <template v-slot:Rules>
      <Rules />
    </template>
  </TabbedWindow>
</template>
