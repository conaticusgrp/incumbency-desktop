<script setup lang="ts">
  import type { ChartDataset } from "chart.js";
  import LineGraph from "../../graphs/LineGraph.vue";
  import { ref, watch } from "vue";
  const props = defineProps<{ data: ProjectedGraphData }>();
  const datasets = ref<ChartDataset<'line', number[]>[]>([]);

  const pushData = (data: ProjectedGraphData) => {
    datasets.value.push({
      label: "Actual",
      data: data.actual,
      borderColor: '#5B8211',
      backgroundColor: '#5B8211',
      pointBorderWidth: 0,
    });

    if (data.predicted) { 
      datasets.value.push({
        label: "Projected",
        data: data.predicted,
        borderColor: '#A72E2E',
        backgroundColor: '#A72E2E',
        borderDash: [5, 5],
        pointBorderWidth: 0,
      });
    }
  }
  const keyI = ref(0);
  watch(() => props.data.actual.length, () => {
    datasets.value = [];
    pushData(props.data);
    keyI.value++;
  }, { immediate: true });
</script>

<template>
  <LineGraph
    title=''
    titleX='Day'
    titleY='Amount'
    :key="keyI"
    :data="{ labels: data.labels, datasets }">
  </LineGraph>
</template>