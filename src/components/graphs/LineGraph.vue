<script setup lang="ts">
/** EXAMPLE USAGE
  <LineGraph 
      title="Business Amount"
      titleX='Day'
      titleY='Amount'
      data="{{
      labels: [
          'Day 1', 'Day 2', 'Day 3',
          'Day 4', 'Day 5', 'Day 6',
          'Day 7',
      ],
      datasets: [{
          label: 'Actual Business Amount',
          data: [
              20, 30, 40, 
              50, 60, 70,
              80
          ],
          borderColor: '#5B8211',
          pointBorderWidth: 0,
      },{
          label: 'Expected Business Amount',
          data: [
              30, 20, 40,
              30, 50, 40,
              60
          ],
          borderColor: '#A72E2E',
          borderDash: [5, 5],
          pointBorderWidth: 0,
      }]
  }}"></LineGraph>
 
*/
import type { ChartData, ChartOptions, Point } from "chart.js";
import { PropType, ref, watch } from "vue";
import { Line } from "vue-chartjs";

const props = defineProps({
    data: {
        type: Object as PropType<
            ChartData<"line", (number | Point | null)[], unknown>
        >,
        required: true,
    },
    title: {
        type: String,
        required: true,
    },
    titleX: {
        type: String,
        default: "",
    },
    titleY: {
        type: String,
        default: "",
    },
});
const data = ref(props.data);
const keyI = ref(0);
const font = {
    size: 20,
    weight: "bold",
    family: "Fira Code",
};
const axisFont = {
    ...font,
    size: font.size - 5,
};
const config: ChartOptions<"line"> = {
    scales: {
        x: {
            title: {
                text: props.titleX,
                display: props.titleX.length > 0,
                font: axisFont,
            },
        },
        y: {
            title: {
                text: props.titleY,
                display: props.titleY.length > 0,
                font: axisFont,
            },
        },
    },
    interaction: {
        mode: "index",
        intersect: false,
    },
    plugins: {
        legend: {
            display: false,
        },
        title: {
            align: "center",
            text: props.title,
            display: true,
            color: "white",
            font: {
                ...font,
            },
        },
    },
};

const watchData = () => props.data.datasets.length;
watch(
    watchData,
    () => {
        data.value = props.data;
        keyI.value += 1;
    },
    { immediate: true }
);
</script>

<template>
    <Line :key="keyI" :data="data" :options="config"></Line>
</template>
