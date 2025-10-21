import { useEffect, useRef } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export function useTauriEvent<T>(
  eventName: string,
  handler: (payload: T) => void
) {
  const handlerRef = useRef(handler);
  const unlistenRef = useRef<UnlistenFn | undefined>();

  // Update ref when handler changes
  useEffect(() => {
    handlerRef.current = handler;
  }, [handler]);

  useEffect(() => {
    // Set up the listener
    const setupListener = async () => {
      const unlisten = await listen<T>(eventName, (event) => {
        handlerRef.current(event.payload);
      });
      unlistenRef.current = unlisten;
    };

    setupListener();

    return () => {
      // Call unlisten if it's been set up
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = undefined;
      }
    };
  }, [eventName]);
}
