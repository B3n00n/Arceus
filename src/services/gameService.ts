import { invoke } from '@tauri-apps/api/core';
import type { GameState } from '../types/game.types';

// Backend config (not exposed to frontend)
interface GameConfigDto {
  name: string;
  exePath: string;
  contentPath: string;
  packageName: string;
}

export class GameService {
  static async startGame(config: GameConfigDto): Promise<GameState> {
    return await invoke<GameState>('start_game', {
      configDto: config,
    });
  }

  static async getCurrentGame(): Promise<GameState | null> {
    return await invoke<GameState | null>('get_current_game');
  }

  static async stopGame(): Promise<void> {
    await invoke('stop_game');
  }
}
