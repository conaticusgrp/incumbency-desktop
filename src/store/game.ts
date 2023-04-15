import { defineStore } from 'pinia';

export enum GameState {
  MainMenu = 'MainMenu',
  NewGameMenu = 'NewGameMenu',
  LoadGameMenu = 'LoadGameMenu',
  SettingsMenu = 'SettingsMenu',
  Singleplayer = 'Singleplayer',
}

export const useGameStore = defineStore('game', {
  state: () => ({ state: GameState.LoadGameMenu }),
  actions: {
    goto(state: GameState) {
      this.state = state;
    },
  },
}); 
