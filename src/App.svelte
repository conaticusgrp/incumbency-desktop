<script lang="ts">
    import MainMenu from "./components/views/MainMenu.svelte";
    import NewGameMenu from "./components/views/NewGameMenu.svelte";
    import LoadGameMenu from "./components/views/LoadGameMenu.svelte";
    import MultiplayerMenu from "./components/views/MultiplayerMenu.svelte";
    import SettingsMenu from "./components/views/SettingsMenu.svelte";
    import Credits from "./components/views/Credits.svelte";
    import Loading from "./components/game/desktop/Loading.svelte";
    import SingleplayerGame from "./components/views/SingleplayerGame.svelte";
    import MultiplayerGame from "./components/views/MultiplayerGame.svelte";

    import { appState } from "./stores/appState";

    type AnyAppState =
        | typeof MainMenu
        | typeof NewGameMenu
        | typeof LoadGameMenu
        | typeof MultiplayerMenu
        | typeof SettingsMenu
        | typeof Credits
        | typeof Loading
        | typeof SingleplayerGame
        | typeof MultiplayerGame;

    let currentComponent: AnyAppState | null = null;

    appState.subscribe((value) => {
        switch (value) {
            case "MainMenu":
                currentComponent = MainMenu;
                break;

            case "NewGameMenu":
                currentComponent = NewGameMenu;
                break;

            case "LoadGameMenu":
                currentComponent = LoadGameMenu;
                break;

            case "MultiplayerMenu":
                currentComponent = MultiplayerMenu;
                break;

            case "SettingsMenu":
                currentComponent = SettingsMenu;
                break;

            case "Credits":
                currentComponent = Credits;
                break;

            case "Singleplayer":
                currentComponent = SingleplayerGame;
                break;

            case "Multiplayer":
                currentComponent = MultiplayerGame;
                break;

            default:
                currentComponent = null;
                console.error(
                    `Impossible condition - no such app state: ${value}`
                );
                break;
        }
    });

    document.addEventListener("contextmenu", (event) => event.preventDefault());
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
