import { writable, type Writable } from "svelte/store";

type AppState = "MainMenu" | "NewGameMenu" | "LoadGameMenu" | "MultiplayerMenu" | "SettingsMenu" | "Credits" | "Loading" | "Singleplayer" | "Multiplayer";

const appState: Writable<AppState> = writable<AppState>("Loading");

export { type AppState, appState };
