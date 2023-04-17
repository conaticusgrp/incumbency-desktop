import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

export enum GameState {
  MainMenu = 'MainMenu',
  NewGameMenu = 'NewGameMenu',
  LoadGameMenu = 'LoadGameMenu',
  SettingsMenu = 'SettingsMenu',
  Singleplayer = 'Singleplayer',
}

export const useGameStore = defineStore('game', () => {
  const state = ref({ state: GameState.MainMenu, generated: false });

  const goto = (newState: GameState) => state.value.state = newState;
  const setIsReady = () => state.value.generated = true;
  const generated = computed(() => state.value.generated);

  return { state, goto, setIsReady, generated }
}); 
