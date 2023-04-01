<script lang="ts">
  import type { ChartDataset } from "chart.js";
  import LineGraph from "../LineGraph.svelte";

  type Data = {
    label: string;
    data: number[];
  }
  export let expected: Data;
  export let actual: Data;

  // TODO(dylhack): Convert to days/weeks/months/years 
  //                based on what the backend gives us
  const labels = ['Day 1', 'Day 2', 'Day 3', 'Day 4', 'Day 5', 'Day 6', 'Day 7'];
  // TODO(dylhack): Get actual data from backend
  const datasets: ChartDataset<'line', number[]>[] = [{
    label: actual.label,
    data: actual.data,
    borderColor: '#5B8211',
    pointBorderWidth: 0,
  },
  {
    label: expected.label,
    data: expected.data,
    borderColor: '#A72E2E',
    borderDash: [5, 5],
    pointBorderWidth: 0,
  }];
</script>

<LineGraph
    title="Government Balance"
    titleX='Day'
    titleY='Amount'
    data="{{
    labels: labels,
    datasets: datasets,
}}"></LineGraph>