import { create } from 'zustand';
import type { GameState } from '../types/game.types';
import { eventService } from '../services/eventService';

interface GameStoreState {
  currentGame: GameState | null;

  setCurrentGame: (game: GameState | null) => void;
  clearCurrentGame: () => void;
}

export const useGameStore = create<GameStoreState>((set) => ({
  currentGame: null,

  setCurrentGame: (game) => set({ currentGame: game }),

  clearCurrentGame: () => set({ currentGame: null }),
}));

eventService.subscribe((event) => {
  const store = useGameStore.getState();

  switch (event.type) {
    case 'gameStarted':
      store.setCurrentGame({
        gameName: event.gameName,
      });
      break;

    case 'gameStopped':
      store.clearCurrentGame();
      break;
  }
});
