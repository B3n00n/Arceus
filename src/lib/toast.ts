import { toast as sonnerToast } from 'sonner';
import { useToastHistoryStore } from '@/stores/toastHistoryStore';

interface ToastOptions {
  description?: string;
}

const addToHistory = (
  type: 'success' | 'error' | 'info' | 'warning',
  message: string,
  description?: string
) => {
  useToastHistoryStore.getState().addToast(type, message, description);
};

export const toast = {
  success: (message: string, options?: ToastOptions) => {
    addToHistory('success', message, options?.description);
    const { isOpen } = useToastHistoryStore.getState();
    if (!isOpen) {
      return sonnerToast.success(message, options);
    }
  },

  error: (message: string, options?: ToastOptions) => {
    addToHistory('error', message, options?.description);
    const { isOpen } = useToastHistoryStore.getState();
    if (!isOpen) {
      return sonnerToast.error(message, options);
    }
  },

  info: (message: string, options?: ToastOptions) => {
    addToHistory('info', message, options?.description);
    const { isOpen } = useToastHistoryStore.getState();
    if (!isOpen) {
      return sonnerToast.info(message, options);
    }
  },

  warning: (message: string, options?: ToastOptions) => {
    addToHistory('warning', message, options?.description);
    const { isOpen } = useToastHistoryStore.getState();
    if (!isOpen) {
      return sonnerToast(message, { ...options, icon: '⚠️' });
    }
  },
};
