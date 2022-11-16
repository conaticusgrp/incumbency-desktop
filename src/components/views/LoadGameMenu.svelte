<script lang="ts">
  import { AppState, appState } from "../../stores/appState"
  import { invoke } from "@tauri-apps/api/tauri"

  let saves: string[] = []

  const get_saves = async () => {
    saves = (await invoke("list_saves")) as string[]
  }
  get_saves()
</script>

<main>
  <div class="center" style="height: 100%; flex-direction: column;">
    <h2>Load Game</h2>
    <ul class="horizontal_scroll">
      {#each saves as save}
        <!-- change this to open game view -->
        <div
          on:mouseup={() => appState.set(AppState.MAIN_MENU)}
          class="save_card"
        >
          {save}
        </div>
      {/each}
    </ul>
  </div>
  <button class="button_back" on:click={() => appState.set(AppState.MAIN_MENU)}
    >Back</button
  >
</main>

<style>
  ::-webkit-scrollbar {
    background: #1a1a1a30;
    border-radius: 2rem;
  }
  ::-webkit-scrollbar-thumb {
    background: #1a1a1a;
    border-radius: 2rem;
  }

  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .horizontal_scroll {
    height: 50%;
    width: 100%;
    display: flex;
    align-items: center;

    overflow: auto;
    white-space: nowrap;
  }

  .save_card {
    flex-grow: 0;
    flex-shrink: 0;
    background-color: #1a1a1a;
    border-radius: var(--app-border-radius);

    margin: 0.1rem 1rem;
    width: 20rem;
    height: 25rem;

    /* in case we add cover images */
    mask: linear-gradient(to bottom, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 99%);
  }

  .save_card:hover {
    filter: brightness(0.8);
  }

  .button_back {
    position: absolute;
    bottom: 0.5rem;
    left: 50%;
    transform: translate(-50%, 0);
  }
</style>
