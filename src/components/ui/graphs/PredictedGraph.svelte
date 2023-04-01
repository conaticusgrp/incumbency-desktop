<script lang="ts">
  import type { ChartDataset } from "chart.js";
  import LineGraph from "../LineGraph.svelte";

  type Data = {
    label: string;
    data: number[];
  }
  export let title: string;
  export let actual: Data;
  export let predicted: Data = {
    data: [],
    label: "",
  };

  const getLabels = (): number[] => {
    if (predicted) {
      return Array.from(Array(predicted.data.length).keys());
    }
    return Array.from(Array(actual.data.length).keys());
  }

  const labels = getLabels();
  // TODO(dylhack): Get actual data from backend
  const datasets: ChartDataset<'line', number[]>[] = [{
    label: actual.label,
    data: actual.data,
    borderColor: '#5B8211',
    pointBorderWidth: 0,
  }];

  if (predicted.data.length > 0) { 
    datasets.push({
      label: predicted.label,
      data: predicted.data,
      borderColor: '#A72E2E',
      borderDash: [5, 5],
      pointBorderWidth: 0,
    });
  }
</script>

<LineGraph
    title={title}
    titleX='Day'
    titleY='Amount'
    data="{{
    labels: labels,
    datasets: datasets,
}}"></LineGraph>
