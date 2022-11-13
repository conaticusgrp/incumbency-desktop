import { writable, type Writable } from 'svelte/store';

export enum MenuState {
    MAIN,
    NEW_GAME,
    LOAD_GAME,
    MULTIPLAYER
}

export const menuState: Writable<MenuState> = writable<MenuState>(MenuState.MAIN);
