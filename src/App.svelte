<script lang="ts">

  import MainMenu from './components/views/MainMenu.svelte';
  import NewGameMenu from './components/views/NewGameMenu.svelte';
  import LoadGameMenu from './components/views/LoadGameMenu.svelte';
  import SettingsMenu from './components/views/SettingsMenu.svelte';
  
  import { appState, AppState } from './stores/appState'

  let state: AppState;

  appState.subscribe(value => {
    state = value;
  });

</script>

<main>

  {#if state == AppState.MAIN_MENU}

  <MainMenu />

  {:else if state == AppState.NEW_GAME_MENU}

  <NewGameMenu />

  {:else if state == AppState.LOAD_GAME_MENU}

  <LoadGameMenu />

  {:else if state == AppState.MULTIPLAYER_MENU}

  <span>Not implemented</span>
  
  {:else if state == AppState.SETTINGS_MENU}
  
  <SettingsMenu />
  
  {:else if state == AppState.CREDITS}
  
  <span>Not implemented</span>
  
  {:else if state == AppState.SINGLEPLAYER}
  
  <span>Not implemented</span>
  
  {:else if state == AppState.MULTIPLAYER}
  
  <span>Not implemented</span>
  
  {:else}

  <p>Undefined app state</p>

  {/if}
  
</main>

<style>
  
  main {
    width: 100%;
    height: 100%;
  }

</style>
