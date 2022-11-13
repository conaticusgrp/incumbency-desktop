import { writable, type Writable } from 'svelte/store';

export enum AppState {
    MENU,
    SINGLEPLAYER,
    MULTIPLAYER
}

export const appState: Writable<AppState> = writable<AppState>(AppState.MENU);
