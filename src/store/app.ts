import { defineStore } from 'pinia';

export enum AppState {
  MainMenu = 'MainMenu',
  NewGameMenu = 'NewGameMenu',
  LoadGameMenu = 'LoadGameMenu',
  SettingsMenu = 'SettingsMenu',
  Singleplayer = 'Singleplayer',
}

export const useAppStore = defineStore('app', {
  state: () => ({ state: AppState.LoadGameMenu }),
  actions: {
    setState(state: AppState) {
      this.state = state;
    },
  },
});

