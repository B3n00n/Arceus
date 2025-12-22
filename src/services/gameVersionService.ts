import { invoke } from '@tauri-apps/api/core';

export interface GameStatus {
  gameId: number;
  gameName: string;
  installedVersion: string | null;
  assignedVersion: string;
  assignedVersionId: number;
  updateAvailable: boolean;
  downloadProgress: DownloadProgress | null;
  online: boolean;
  lastSynced: string | null;
}

export interface DownloadProgress {
  percentage: number;
}

export const gameVersionService = {
  /**
   * Get list of all games with their version status
   */
  async getGameList(): Promise<GameStatus[]> {
    return await invoke('get_game_list');
  },

  /**
   * Download and install a game (or update it)
   */
  async downloadGame(gameId: number): Promise<void> {
    await invoke('download_game', { gameId });
  },

  /**
   * Get download progress for a specific game
   */
  async getDownloadProgress(gameId: number): Promise<DownloadProgress | null> {
    return await invoke('get_download_progress', { gameId });
  },

  /**
   * Cancel an ongoing download
   */
  async cancelDownload(gameId: number): Promise<void> {
    await invoke('cancel_download', { gameId });
  },

  /**
   * Force refresh games from server (requires internet connection)
   */
  async forceRefresh(): Promise<GameStatus[]> {
    return await invoke('force_refresh_games');
  },
};
