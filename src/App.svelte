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

  type AnyAppState = typeof MainMenu |
                      typeof NewGameMenu |
                      typeof LoadGameMenu |
                      typeof MultiplayerMenu |
                      typeof SettingsMenu |
                      typeof Credits |
                      typeof SingleplayerGame |
                      typeof MultiplayerGame;

  let currentComponent: AnyAppState | null = null;

  appState.subscribe(value => {
    switch (value) {
      case AppState.MAIN_MENU:
        currentComponent = MainMenu;
        break;

      case AppState.NEW_GAME_MENU:
        currentComponent = NewGameMenu;
        break;

      case AppState.LOAD_GAME_MENU:
        currentComponent = LoadGameMenu;
        break;

      case AppState.MULTIPLAYER_MENU:
        currentComponent = MultiplayerMenu;
        break;

      case AppState.SETTINGS_MENU:
        currentComponent = SettingsMenu;
        break;

      case AppState.CREDITS:
        currentComponent = Credits;
        break;

      case AppState.SINGLEPLAYER:
        currentComponent = SingleplayerGame;
        break;

      case AppState.MULTIPLAYER:
        currentComponent = MultiplayerGame;
        break;

      default:
        currentComponent = null;
        console.error(`No such app state: ${value}`);
        break;
    };
  });

</script>

<main>

  {#if currentComponent != null}

  <svelte:component this={currentComponent} />
  
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
