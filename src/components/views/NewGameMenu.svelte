<script lang="ts">

  import { invoke } from "@tauri-apps/api/tauri";
  import { appState } from "../../stores/appState";

  let name = "";
  let saveAlreadyExists = false;

  const checkAlreadyExists = async () => {
    saveAlreadyExists = await invoke("check_save_exists", { name });
  }

  const createGame = async () => {
    if (name.length === 0 || saveAlreadyExists) return;
    await invoke("create_game", { name });
  } 
  
</script>

<main>

  <h2>New Game</h2>

  {#if saveAlreadyExists}
    <p style="color: red">Save with name '{name}' already exists.</p>
  {/if}

  <div class="save_name">
    <label for="name">Enter save name: </label>
    <input type="text" required bind:value={name} min="1" max="30" on:keyup={async () => await checkAlreadyExists()} />
  </div>

  <div>
    <button on:click={() => appState.set('MainMenu')}>Back</button>
    <button on:click={createGame} type="submit">Create!</button>
  </div>

</main>

<style>

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100%;
    height: 100%;
  }

</style>
