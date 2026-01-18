import { useToastHistoryStore } from '@/stores/toastHistoryStore';
import { cn } from '@/lib/cn';
import { CheckCircle, XCircle, Info, AlertTriangle, Trash2, X } from 'lucide-react';
import { Button } from './ui/button';

export function ToastHistory() {
  const { toasts, isOpen, setIsOpen, clearHistory } = useToastHistoryStore();

  const getIcon = (type: string) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="h-4 w-4 text-success-default" />;
      case 'error':
        return <XCircle className="h-4 w-4 text-error-default" />;
      case 'info':
        return <Info className="h-4 w-4 text-info-default" />;
      case 'warning':
        return <AlertTriangle className="h-4 w-4 text-warning-default" />;
      default:
        return <Info className="h-4 w-4 text-grey-300" />;
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'success':
        return 'text-success-default';
      case 'error':
        return 'text-error-default';
      case 'info':
        return 'text-info-default';
      case 'warning':
        return 'text-warning-default';
      default:
        return 'text-grey-300';
    }
  };

  const formatTime = (date: Date) => {
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;

    return date.toLocaleDateString();
  };

  return (
    <>
      {/* Invisible backdrop to catch outside clicks */}
      {isOpen && (
        <div
          className="fixed inset-0 z-[45]"
          onClick={() => setIsOpen(false)}
        />
      )}

      {/* Floating notification panel */}
      <div
        className={cn(
          "fixed top-20 right-6 w-96 max-h-[600px] bg-grey-800 border border-grey-700 rounded-lg shadow-2xl z-[50] flex flex-col overflow-hidden",
          "transition-all duration-200 ease-out origin-top-right",
          isOpen ? "opacity-100 scale-100" : "opacity-0 scale-95 pointer-events-none"
        )}
      >
        {/* Header */}
        <div className="p-4 border-b border-grey-700 flex items-center justify-between">
          <div>
            <h3 className="font-semibold text-white text-lg">Notifications</h3>
            <p className="text-xs text-grey-300 mt-0.5">
              {toasts.length === 0 ? 'No notifications' : `${toasts.length} notification${toasts.length !== 1 ? 's' : ''}`}
            </p>
          </div>
          <div className="flex items-center gap-2">
            {toasts.length > 0 && (
              <Button
                variant="ghost"
                size="sm"
                onClick={clearHistory}
                className="text-grey-300 hover:text-white"
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            )}
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setIsOpen(false)}
              className="text-grey-300 hover:text-white"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Toast List */}
        <div className="flex-1 overflow-y-auto">
          {toasts.length === 0 ? (
            <div className="flex flex-col items-center justify-center p-4 h-full text-grey-300">
              <Info className="h-12 w-12 mb-3 opacity-50" />
              <p className="text-sm">No notifications yet</p>
            </div>
          ) : (
            <div className="divide-y divide-grey-700">
              {toasts.map((toast) => (
                <div
                  key={toast.id}
                  className="p-3 hover:bg-grey-700 transition-colors"
                >
                  <div className="flex items-start gap-3">
                    <div className="mt-0.5">{getIcon(toast.type)}</div>
                    <div className="flex-1 min-w-0">
                      <p className={cn('text-sm font-medium', getTypeColor(toast.type))}>
                        {toast.message}
                      </p>
                      {toast.description && (
                        <p className="text-xs text-grey-200 mt-1">
                          {toast.description}
                        </p>
                      )}
                      <p className="text-xs text-grey-300 mt-1.5">
                        {formatTime(toast.timestamp)}
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </>
  );
}
