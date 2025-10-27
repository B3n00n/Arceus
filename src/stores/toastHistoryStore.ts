import { create } from 'zustand';
import { toast } from 'sonner';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface ToastHistoryItem {
  id: string;
  type: ToastType;
  message: string;
  description?: string;
  timestamp: Date;
}

interface ToastHistoryState {
  toasts: ToastHistoryItem[];
  isOpen: boolean;
  unreadCount: number;
  addToast: (type: ToastType, message: string, description?: string) => void;
  clearHistory: () => void;
  togglePanel: () => void;
  setIsOpen: (isOpen: boolean) => void;
  markAsRead: () => void;
}

const MAX_TOASTS = 30;

export const useToastHistoryStore = create<ToastHistoryState>((set) => ({
  toasts: [],
  isOpen: false,
  unreadCount: 0,

  addToast: (type, message, description) => {
    const newToast: ToastHistoryItem = {
      id: `${Date.now()}-${Math.random()}`,
      type,
      message,
      description,
      timestamp: new Date(),
    };

    set((state) => ({
      toasts: [newToast, ...state.toasts].slice(0, MAX_TOASTS),
      unreadCount: state.isOpen ? state.unreadCount : state.unreadCount + 1,
    }));
  },

  clearHistory: () => set({ toasts: [], unreadCount: 0 }),

  togglePanel: () => {
    const currentState = useToastHistoryStore.getState();
    const newIsOpen = !currentState.isOpen;

    // Dismiss all active toasts when opening the panel
    if (newIsOpen) {
      toast.dismiss();
    }

    set({
      isOpen: newIsOpen,
      unreadCount: newIsOpen ? 0 : currentState.unreadCount,
    });
  },

  setIsOpen: (isOpen) => {
    // Dismiss all active toasts when opening the panel
    if (isOpen) {
      toast.dismiss();
    }

    set({ isOpen, unreadCount: isOpen ? 0 : undefined });
  },

  markAsRead: () => set({ unreadCount: 0 }),
}));
