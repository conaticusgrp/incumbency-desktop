<script setup lang="ts">
  import type { ChartDataset } from "chart.js";
  import LineGraph from "../../graphs/LineGraph.vue";

  const props = defineProps<{ data: ProjectedGraphData }>();
  const datasets: ChartDataset<'line', number[]>[] = [{
    label: "Actual",
    data: props.data.actual,
    borderColor: '#5B8211',
    backgroundColor: '#5B8211',
    pointBorderWidth: 0,
  }];

  if (props.data.predicted) { 
    datasets.push({
      label: "Projected",
      data: props.data.predicted,
      borderColor: '#A72E2E',
      backgroundColor: '#A72E2E',
      borderDash: [5, 5],
      pointBorderWidth: 0,
    });
  }
</script>

<template>
  <LineGraph
    title=''
    titleX='Day'
    titleY='Amount'
    :data="{ labels: data.labels, datasets }">
  </LineGraph>
</template>