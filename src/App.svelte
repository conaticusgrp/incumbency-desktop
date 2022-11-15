<script lang="ts">

  import MainMenu from './components/views/MainMenu.svelte';
  import SettingsPage from './components/views/SettingsPage.svelte';
  import { appState, AppState } from './stores/appState'

  let state: AppState;

  appState.subscribe(value => {
    state = value;
  });

</script>

<main>

  {#if state == AppState.MAIN_MENU}

  <MainMenu />

  {:else if state == AppState.SETTINGS_MENU}
    <SettingsPage />
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
