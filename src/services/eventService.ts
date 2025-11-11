import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { ArceusEvent } from '@/types/events.types';
import { toast } from '@/lib/toast';

type EventCallback = (event: ArceusEvent) => void;

class EventService {
  private unlisten: UnlistenFn | null = null;
  private callbacks: Set<EventCallback> = new Set();

  async initialize() {
    if (this.unlisten) {
      console.warn('EventService already initialized');
      return;
    }

    this.unlisten = await listen<ArceusEvent>('arceus://event', (event) => {
      this.handleEvent(event.payload);
    });
  }

  subscribe(callback: EventCallback): () => void {
    this.callbacks.add(callback);

    return () => {
      this.callbacks.delete(callback);
    };
  }


  private handleEvent(event: ArceusEvent) {
    this.handleNotifications(event);

    this.callbacks.forEach(callback => callback(event));
  }

  private handleNotifications(event: ArceusEvent) {
    switch (event.type) {
      case 'deviceConnected':
        const deviceName = event.device.info.customName || 'Quest';
        toast.success(`${deviceName}: Connected`);
        break;

      case 'deviceDisconnected':
        toast.info(`Device disconnected`);
        break;

      case 'deviceNameChanged':
        const displayName = event.newName || event.serial;
        toast.success(`Renamed to "${displayName}"`);
        break;

      case 'commandExecuted':
        if (event.result.success) {
          toast.success(event.result.message);
        } else {
          toast.error(event.result.message);
        }
        break;

      case 'error':
        toast.error(event.message, {
          description: event.context || undefined,
        });
        break;

      case 'info':
        toast.info(event.message);
        break;

      case 'gameStarted':
        toast.success(`${event.gameName}: Started`);
        break;

      case 'gameStopped':
        toast.info(`${event.gameName}: Stopped`);
        break;
    }
  }

  async destroy() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    this.callbacks.clear();
  }
}

export const eventService = new EventService();
