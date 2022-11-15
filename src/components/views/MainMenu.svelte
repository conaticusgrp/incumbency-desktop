<script lang="ts">

  import { appState, AppState } from "../../stores/appState";

  import LoadGameMenu from "./LoadGameMenu.svelte";
  import NewGameMenu from "./NewGameMenu.svelte";
  import Logo from "../../assets/logo.svg";

  let state: AppState;
  let mainMenu: HTMLElement;

  const showMainMenu = (): void => {
    // Remove the shading
    console.log("main menu");
  }

  const showNewGame = (): void => {
    mainMenu.classList.add('shaded');
        
    // Show new game menu
    console.log("new game");
  }
    
  const showLoadGame = (): void => {
    mainMenu.classList.add('shaded');
        
    // Show load game menu
    console.log("load game");
  }
    
  const showMultiplayer = (): void => {
    // Debug
    alert("Disabled feature");
  }

  appState.subscribe(value => {
    state = value;
    switch (value) {
      case AppState.MAIN_MENU:        showMainMenu();     break;
      case AppState.NEW_GAME_MENU:    showNewGame();      break;
      case AppState.LOAD_GAME_MENU:   showLoadGame();     break;
      default: break;
    }
  });

</script>

<main>

  <div class="main_menu" bind:this={mainMenu}>
        
    <img src={Logo} alt="Logo" class="logo">
        
    <!--
      The buttons will probably be changed to components.
    -->
    <button on:click={() => appState.set(AppState.NEW_GAME_MENU)}>New Game</button>
    <button on:click={() => appState.set(AppState.LOAD_GAME_MENU)}>Load Game</button>
    <button on:click={() => appState.set(AppState.MULTIPLAYER_MENU)} class="disabled">Multiplayer</button>
    <button on:click={() => appState.set(AppState.SETTINGS_MENU)}>Settings</button>
        
  </div>

  {#if state == AppState.NEW_GAME_MENU}

  <div class="menu_content">
    <NewGameMenu />
  </div>

  {:else if state == AppState.LOAD_GAME_MENU}

  <div class="menu_content">
    <LoadGameMenu />
  </div>
    
  {:else if state == AppState.MULTIPLAYER_MENU}

  <div class="menu_content">
    <p>Multiplayer mode not implemented.</p>
  </div>
    
  {:else if state != AppState.MAIN_MENU}

  <div class="menu_content">
    <p>Error: No such menu state: {state}</p>
  </div>

  {/if}


</main>

<style>

  /* Note: Grid could be used to make the layout for the menu page. */

  main {
    /* position: relative; */
    width: 100%;
    height: 100%;
  }
    
  .main_menu {
    /* position: relative; */
    width: 100%;
    height: 100%;
    pointer-events: all;
  }

  .menu_content {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }
    
  .logo {
    margin: 0 auto;
    display: block;
    width: 100%;
    max-width: 650px;
    max-height: 500px;
    margin-top: 4rem;
  }

  /* Note: The "disabled" class should probably be moved to app.css since it could be used in the other parts for the app. */
  .disabled {
    position: relative;
  }
  .disabled:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
  }


</style>