import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { UpdateStatus } from '../types/update.types';

export class UpdateService {
  private statusListener?: UnlistenFn;

  async checkForUpdates(): Promise<UpdateStatus> {
    try {
      return await invoke<UpdateStatus>('check_for_updates');
    } catch (error) {
      return {
        type: 'Error',
        data: { message: error instanceof Error ? error.message : 'Unknown error occurred' }
      } as UpdateStatus;
    }
  }

  async downloadAndInstall(): Promise<void> {
    return await invoke('download_and_install_update');
  }

  async skipUpdate(): Promise<void> {
    return await invoke('skip_update');
  }

  async listenForStatus(callback: (status: UpdateStatus) => void): Promise<UnlistenFn> {
    this.statusListener = await listen<UpdateStatus>('update-status', (event) => {
      callback(event.payload);
    });
    return this.statusListener;
  }

  async cleanup(): Promise<void> {
    if (this.statusListener) {
      this.statusListener();
      this.statusListener = undefined;
    }
  }
}

export const updateService = new UpdateService();