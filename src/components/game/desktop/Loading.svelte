<script lang="ts" context="module">

  interface LoadingStatus {
    main: string,
    substatuses: string[]
  }

</script>

<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  
  const LOADING_STAGE_COUNT = 4;
  const MIN_LOG_DELAY = 250;
  const MAX_LOG_DELAY = 500;
  
  let details: HTMLElement;
  let timerResolve: (() => void) | null = null;
  let progress = -100 / LOADING_STAGE_COUNT;    // %
  let status: LoadingStatus = { main: "", substatuses: [] };

  const randomDelay = async () => {
    await new Promise<void>(resolve => {
      setTimeout(resolve, MIN_LOG_DELAY + (MAX_LOG_DELAY - MIN_LOG_DELAY) * Math.random());
    });
  }

  const log = (msg: string): void => {
    details.innerText += `${msg}\n`;
    // console.trace(msg);
  }

  listen('loading_status', (e) => {
    console.log(e);
    if (timerResolve != null) {
      timerResolve();
      status.substatuses.forEach(m => log(m));
    }

    //@ts-ignore
    status.main = Object.keys(e.payload)[0];
    //@ts-ignore
    status.substatuses = e.payload[status.main];

    progress += 100 / LOADING_STAGE_COUNT;

    new Promise<void>(async (resolve, reject) => {
      timerResolve = resolve;
      for (let i = 0; i < status.substatuses.length; i++) {
        await randomDelay();
        // console.log("status", status.substatuses[i]);
        log(status.substatuses.splice(i, 1)[0]);
      }
      timerResolve = null;
      resolve();
    });
  });

</script>

<main>

  <div class="loading-panel">

    <h1>{status.main}</h1>

    <div class="content">

      <div class="progress">
        <span>Progress</span>
        <div class="progress-bar" style="--bar-width: {Math.min(Math.max(progress, 0), 100)}%;"></div>
      </div>

      <span style="margin-top: 2em;">Details:</span>
      <div
        class="details"
        
      >
        <div style="height: 100%;" bind:this={details}></div>
      </div>

    </div>

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
    display: flex;
    flex-direction: column;
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

  .content {
    display: flex;
    flex-direction: column;
    width: calc(100% - 2 * 1em);
    height: 100%;
    margin: 1em;
    text-align: left;
  }

  .progress-bar {
    position: relative;
    width: 100%;
    height: 5px;
    margin-top: 0.5em;
  }

  .progress-bar:after {
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
    width: 100%;
    height: 100%;
    border: 1px solid var(--color-accent);
    border-radius: 1em;
  }

</style>
