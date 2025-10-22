import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { ArceusEvent } from '@/types/events.types';
import { toast } from 'sonner';

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
        toast.success(`${event.device.info.model} connected`);
        break;

      case 'deviceDisconnected':
        toast.info(`Device disconnected`);
        break;

      case 'commandExecuted':
        if (event.result.success) {
          toast.success(`${event.result.commandType}: ${event.result.message}`);
        } else {
          toast.error(`${event.result.commandType} failed: ${event.result.message}`);
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
