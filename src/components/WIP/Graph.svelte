<script lang="ts">

  import { onMount } from "svelte";

  export let size: { width: Number, height: number };
  export let title: string = "Title";
  export let yAxisName: string = "Y";
  export let values: { key: any, value: number }[];
  export let yRange: [number, number];
  export let yValuesCount: number;

  let initialized = false;
  let thisObj: HTMLElement;
  let canvas: HTMLCanvasElement;
  let yAxis: HTMLElement;
  let ctx: CanvasRenderingContext2D;

  const BOTTOM_PADDING = 5;

  const measureText = (str: string) => {
    const metrics = ctx.measureText(str);
    const actualHeight = metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    return [Math.ceil(metrics.width), actualHeight];
  }

  const renderAxis = () => {
    const [textWidth, textHeight] = measureText(String(yRange[1]));
    ctx.strokeStyle = "rgb(100, 255, 100)";
    ctx.moveTo(textWidth, 0);
    ctx.lineTo(textWidth, canvas.height - textHeight - BOTTOM_PADDING);
    ctx.lineTo(canvas.width, canvas.height - textHeight - BOTTOM_PADDING);
    ctx.stroke();
  }

  const renderXLabels = () => {
    const textWidth = measureText(String(yRange[1]))[0];
    ctx.strokeStyle = "grey";
    for (let i = 0; i < values.length; i++) {
      const x = textWidth + (i / values.length) * canvas.width;
      const y = canvas.height - 1;
      ctx.strokeText(String(values[i].key), x, y);
    }
  }

  const renderYLabels = () => {
    const textHeight = measureText('0')[1];
    yValuesCount = Math.max(yValuesCount, 1);
    ctx.strokeStyle = "grey";
    for (let i = 0; i < yValuesCount; i++) {
      const x = 0;
      const y = (1 - i / yValuesCount) * canvas.height - textHeight / 2;
      const lerped = yRange[0] + (yRange[1] - yRange[0]) * (i / yValuesCount);
      ctx.strokeText(String(lerped), x, y);
    }
  }

  const renderValues = () => {
    const [textWidth, textHeight] = measureText(String(yRange[1]));
    ctx.strokeStyle = "green";
    let prevVal, curVal;
    for (let i = 1; i < values.length; i++) {
      prevVal = values[i - 1];
      curVal = values[i];

      if (prevVal == undefined) {
        continue;
      }

      const curProgress = (curVal.value - yRange[0]) / (yRange[1] - yRange[0]);
      const curX = textWidth + (i / values.length) * canvas.width;
      const curY = (1 - curProgress) * canvas.height - textHeight / 2;
      const prevProgress = (prevVal.value - yRange[0]) / (yRange[1] - yRange[0]);
      const prevX = textWidth + ((i - 1) / values.length) * canvas.width;
      const prevY = (1 - prevProgress) * canvas.height - textHeight / 2;

      ctx.moveTo(prevX, prevY);
      ctx.lineTo(curX, curY);
      ctx.stroke();
    }
  }

  const renderData = () => {
    if (!initialized) return;

    renderValues();
    renderXLabels();
    renderYLabels();
    renderAxis();
  }

  onMount(() => {  
    const thisX = thisObj.getBoundingClientRect().x;
    const canvasX = canvas.getBoundingClientRect().x;
    const canvasOffset = canvasX - thisX;
    const canvasParent = canvas.parentElement;
    if (canvasParent == null) return;
    const middleBoxMarginRightStr = getComputedStyle(canvasParent).marginRight;
    const middleBoxMarginRight = Number(middleBoxMarginRightStr.substring(0, middleBoxMarginRightStr.length - 2));

    //@ts-ignore
    if (middleBoxMarginRight == NaN) {
      console.error(`CSS rule margin-right was of a wrong format: ${middleBoxMarginRightStr}`);
      return;
    }

    canvas.width = thisObj.clientWidth - canvasOffset - middleBoxMarginRight;
    canvas.height = canvasParent.clientHeight;
    const context = canvas.getContext('2d');
    if (context == null) {
      console.error("Couldn't aquire canvas context.");
      return;
    }
    ctx = context;

    initialized = true;
    renderData();
  });

</script>

<main
      style="width: {size.width}px; height: {size.height}px;"
      bind:this={thisObj}
>

  <h2>{title}</h2>
  
  <div class="middle-area">

    <div class="axis-wrapper">
      <span
            style="transform: translateY({yAxis?.clientWidth / 2}px) rotate(-90deg);"
            bind:this={yAxis}
      >
        {yAxisName}
      </span>
    </div>

    <canvas
      width="0"
      height="0"
      bind:this={canvas}
    ></canvas>

  </div>
  
</main>

<style>

  main {
    display: grid;
    grid-template-rows: 2em 1fr 1em;
    box-sizing: border-box;
    background-color: black;
    color: white;
    text-align: center;
  }
  
  h2 {
    margin: 0;
    pointer-events: none;
  }
  
  .middle-area {
    display: flex;
    flex-direction: row;
    margin: 0 1em 0 1em;
/*     background-color: red; */
  }
  
  .axis-wrapper {
    position: relative;
    width: 2em;
/*     background-color: green; */
  }
  
  span {
    position: absolute;
    top: 50%;
    left: 0;
    display: block;
    width: max-content;
    height: 100%;
    transform-origin: 0 0;
    pointer-events: none;
    /*     background-color: red; */
  }
  
  canvas {
/*     background-color: blue; */
  }
  
</style>