import { writable, type Writable } from "svelte/store";

type AppState = "MainMenu" | "NewGameMenu" | "LoadGameMenu" | "MultiplayerMenu" | "SettingsMenu" | "Credits" | "Singleplayer" | "Multiplayer";

const appState: Writable<AppState> = writable<AppState>("Singleplayer");

export { type AppState, appState };
