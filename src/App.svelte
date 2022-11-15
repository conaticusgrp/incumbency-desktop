<script lang="ts">

  import MainMenu from './components/views/MainMenu.svelte';
  import NewGameMenu from './components/views/NewGameMenu.svelte';
  import LoadGameMenu from './components/views/LoadGameMenu.svelte';
  import MultiplayerMenu from './components/views/MultiplayerMenu.svelte';
  import SettingsMenu from './components/views/SettingsMenu.svelte';
  import Credits from './components/views/Credits.svelte';
  import SingleplayerGame from './components/views/SingleplayerGame.svelte';
  import MultiplayerGame from './components/views/MultiplayerGame.svelte';
  
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

  <MultiplayerMenu />
  
  {:else if state == AppState.SETTINGS_MENU}
  
  <SettingsMenu />
  
  {:else if state == AppState.CREDITS}
  
  <Credits />
  
  {:else if state == AppState.SINGLEPLAYER}
  
  <SingleplayerGame />
  
  {:else if state == AppState.MULTIPLAYER}
  
  <MultiplayerGame />
  
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
