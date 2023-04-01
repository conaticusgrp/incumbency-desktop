<script lang="ts">
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
  import type { ChartData, ChartOptions, Point } from 'chart.js';
  import { Line } from 'svelte-chartjs';

  // export let data: ChartData<'line'> 
  export let data: ChartData<"line", (number | Point)[], unknown>;
  export let title: string;
  export let titleX: string = '';
  export let titleY: string = '';
  const font = {
    size: 20,
    weight: 'bold',
    family: 'Fira Code'
  }
  const axisFont = {
    ...font,
    size: font.size - 5,
  }
  const config: ChartOptions<'line'> = {
    scales: {
      x: {
        title: {
          text: titleX,
          display: titleX.length > 0,
          font: axisFont,
        },
      },
      y: {
        title: {
          text: titleY,
          display: titleY.length > 0,
          font: axisFont
        }
      },
    },
    interaction: {
      mode: 'index',
      intersect: false
    },
    plugins: {
      legend: {
        display: false,
      },
      title: {
        align: 'center',
        text: title,
        display: true,
        color: 'white',
        font: {
          ...font,
        },
      }
    }
  };

</script>

<Line data={data} options={config}></Line>
