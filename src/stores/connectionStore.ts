import { create } from 'zustand';

interface ConnectionState {
  isOnline: boolean;
  setIsOnline: (online: boolean) => void;
}

export const useConnectionStore = create<ConnectionState>((set) => ({
  isOnline: true,
  setIsOnline: (online) => set({ isOnline: online }),
}));
