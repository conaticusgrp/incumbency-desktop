<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  
  let progress = 50;    // %

  // TODO: listen for the loading progress events
  

</script>

<main>

  <div class="loading-panel">

    <h1>loading population</h1>

    <section>

      <span>Progress</span>
      <div class="progress" style="--bar-width: {Math.min(Math.max(progress, 0), 100)}%;"></div>

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
    margin: auto;
    border: 1px solid var(--color-accent);
    border-radius: 0.5rem;
  }

  h1 {
    padding: 0.25em 0 0.25em 0;
    color: var(--color-bg);
    background-color: var(--color-accent);
    border-top-left-radius: 0.3rem;
    border-top-right-radius: 0.3rem;
    letter-spacing: 0.25em;
    text-align: center;
    text-transform: uppercase;
    font-weight: bold;
    font-size: 20px;
  }

  section {
    width: calc(100% - 2 * 1em);
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
  }

</style>
