<script lang="ts">
  import type { ChartDataset } from "chart.js";
  import LineGraph from "../LineGraph.svelte";
  import type { Data } from "../../../scripts/data";

  export let title: string;
  export let data: Data<number> = {
    type: 0,
    labels: [],
    actual: {
      label: "Actual",
      data: [],
    },
    predicted: {
      label: "Predicted",
      data: [],
    },
  };

  const dataset: ChartDataset<'line', number[]>[] = [{
    label: data.actual.label,
    data: data.actual.data,
    borderColor: '#5B8211',
    backgroundColor: '#5B8211',
    pointBorderWidth: 0,
  }];

  if (data.predicted) { 
    dataset.push({
      label: data.predicted.label,
      data: data.predicted.data,
      borderColor: '#A72E2E',
      backgroundColor: '#A72E2E',
      borderDash: [5, 5],
      pointBorderWidth: 0,
    });
  }
</script>

<LineGraph
  title={title}
  titleX='Day'
  titleY='Amount'
  data="{{ labels: data.labels, datasets: dataset }}">
</LineGraph>
