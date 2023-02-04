<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  
  const LOADING_STAGE_COUNT = 4;
  
  let details: HTMLElement;
  let timerId: NodeJS.Timer;
  let progress = -100 / LOADING_STAGE_COUNT;    // %
  let status: { main: string, substatuses: string[] } = { main: "", substatuses: [] };
  let currentSubstatusIndex: number = 0;

  listen('loading_status', (e) => {
    //@ts-ignore
    status.main = Object.keys(e.payload)[0];
    //@ts-ignore
    status.substatuses = e.payload[status.main];
    currentSubstatusIndex = 0;

    progress += 100 / LOADING_STAGE_COUNT;
  });

  onMount(() => {
    timerId = setInterval(() => {
      if (status == undefined || status.substatuses == undefined || status.substatuses.length === 0) {
        return;
      }
      currentSubstatusIndex = (currentSubstatusIndex + 1) % status.substatuses.length;
      details.innerText = status.substatuses[currentSubstatusIndex];
    }, 5_000);
  });

  onDestroy(() => {
    clearInterval(timerId);
  });
  

</script>

<main>

  <div class="loading-panel">

    <h1>{status.main}</h1>

    <section>

      <span>Progress</span>
      <div class="progress" style="--bar-width: {Math.min(Math.max(progress, 0), 100)}%;"></div>

      <div
        class="details"
        bind:this={details}
      ></div>

    </section>

  </div>

</main>

<style>

  main {
    display: flex;
    align-items: center;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 100;
    isolation: isolate;
    background-color: var(--color-bg);
  }

  .loading-panel {
    width: 50%;
    height: 35%;
    min-height: 300px;
    margin: auto;
    border: 1px solid var(--color-accent);
    border-radius: 0.5rem;
  }

  h1 {
    width: calc(100% - 2 * 0.5em);
    padding: 0.25em 0.5em 0.25em 0.5em;
    color: var(--color-bg);
    background-color: var(--color-accent);
    border-top-left-radius: 0.3rem;
    border-top-right-radius: 0.3rem;
    letter-spacing: 0.25em;
    text-align: center;
    text-transform: uppercase;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-weight: bold;
    font-size: 20px;
  }

  section {
    width: calc(100% - 2 * 1em);
    height: calc(100% - 2 * 0.25em - 1em);
    margin: 1em;
    text-align: left;
  }

  .progress {
    position: relative;
    width: 100%;
    height: 5px;
    margin-top: 0.5em;
  }

  .progress:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: var(--bar-width);
    height: 100%;
    background-color: var(--color-accent);
    transition: width 2s ease;
  }

  .details {
    width: calc(100% - 2 * 2em);
    margin-top: 2em;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .details:before {
    content: 'Details: ';
  }

</style>
