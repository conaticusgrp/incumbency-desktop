<script lang="ts">
    
    import { menuState, MenuState } from "../../stores/menuState";
    import { appState, AppState } from "../../stores/appState";

    import LoadGameMenu from "./LoadGameMenu.svelte";
    import NewGameMenu from "./NewGameMenu.svelte";
    import Logo from "../../assets/logo.svg";

    let state: MenuState;
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

    menuState.subscribe(value => {
        state = value;
        switch (value) {
            case MenuState.MAIN:        showMainMenu();     break;
            case MenuState.NEW_GAME:    showNewGame();      break;
            case MenuState.LOAD_GAME:   showLoadGame();     break;
            case MenuState.MULTIPLAYER: showMultiplayer();  break;
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
        <button on:click={() => menuState.set(MenuState.NEW_GAME)}>New Game</button>
        <button on:click={() => menuState.set(MenuState.LOAD_GAME)}>Load Game</button>
        <button class="disabled" on:click={() => menuState.set(MenuState.MULTIPLAYER)}>Multiplayer</button>
        <button on:click={() => appState.set(AppState.SETTINGS)}>Settings</button>
        
    </div>

    {#if state == MenuState.NEW_GAME}

    <div class="menu_content">
        <NewGameMenu />
    </div>

    {:else if state == MenuState.LOAD_GAME}

    <div class="menu_content">
        <LoadGameMenu />
    </div>
    
    {:else if state == MenuState.MULTIPLAYER}

    <div class="menu_content">
        <p>Multiplayer mode not implemented.</p>
    </div>
    
    {:else if state != MenuState.MAIN}

    <div class="menu_content">
        <p>Error: No such menu state: {state}</p>
    </div>

    {/if}


</main>

<!-- A workaround, so Svelte doesn't delete .main_menu.shaded selector -->
{#if false}
<span class="main_menu shaded"></span>
{/if}

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

    .main_menu:after {
        content: '';
        pointer-events: none;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: transparent;
    }
    
    .main_menu.shaded {
        pointer-events: none;
    }
    
    .main_menu.shaded:after {
        background-color: rgba(0, 0, 0, 0.5);
    }

    .menu_content {
        /* --menu-content- */
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
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

    .logo {
        margin: 0 auto;
        display: block;
        width: 100%;
        max-width: 650px;
        max-height: 500px;
        margin-top: 4rem;
    }
</style>