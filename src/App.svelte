<script lang="ts">

    import MainMenu from './components/views/MainMenu.svelte';
    import { appState, AppState } from './stores/appState'

    let state: AppState;

    appState.subscribe(value => {
      state = value;
    });

</script>

<main>

    {#if state == AppState.MENU}

    <MainMenu />

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
