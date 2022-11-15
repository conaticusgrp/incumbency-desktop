import { writable, type Writable } from 'svelte/store';

export enum AppState {
  MAIN_MENU,
  NEW_GAME_MENU,
  LOAD_GAME_MENU,
  MULTIPLAYER_MENU,
  SETTINGS_MENU,
  CREDITS,

  SINGLEPLAYER,
  MULTIPLAYER
}

export const appState: Writable<AppState> = writable<AppState>(AppState.MAIN_MENU);
