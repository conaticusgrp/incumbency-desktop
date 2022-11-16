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
  <p>load game</p>
  <div class="horizontal_scroll">
    {#each saves as save}
      <!-- change this to open game view -->
      <div
        on:mouseup={() => appState.set(AppState.MAIN_MENU)}
        class="save_card"
      >
        {save}
      </div>
    {/each}
  </div>
  <button class="button_back" on:click={() => appState.set(AppState.MAIN_MENU)}
    >Back</button
  >
</main>

<style>
  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .horizontal_scroll {
    display: flex;
    flex-wrap: nowrap;
    overflow-x: auto;
  }

  .save_card {
    display: inline-block;
    background-color: #1a1a1a;
    margin: 0 1rem;
    width: 18rem;
    width: 300px;
    height: 25rem;
  }

  button:hover:enabled {
    border: none;
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
