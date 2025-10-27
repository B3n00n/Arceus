import { Bell } from 'lucide-react';
import { useToastHistoryStore } from '@/stores/toastHistoryStore';
import { ToastHistory } from '@/components/ToastHistory';

export function Header() {
  const { togglePanel, unreadCount } = useToastHistoryStore();

  return (
    <>
      <header className="sticky top-0 z-30 h-16 border-b border-discord-dark-2 bg-discord-dark-3/95 backdrop-blur supports-[backdrop-filter]:bg-discord-dark-3/75">
        <div className="flex h-full items-center justify-between px-6">
          <div>{/* Empty - can be used for page-specific content */}</div>

          {/* Notification Bell */}
          <button
            onClick={togglePanel}
            className="relative p-2 rounded-lg hover:bg-discord-dark-2 transition-colors group"
          >
            <Bell className="h-5 w-5 text-gray-400 group-hover:text-white transition-colors" />
            {unreadCount > 0 && (
              <span className="absolute -top-1 -right-1 h-5 w-5 bg-red-500 rounded-full flex items-center justify-center">
                <span className="text-xs font-semibold text-white">
                  {unreadCount > 9 ? '9+' : unreadCount}
                </span>
              </span>
            )}
          </button>
        </div>
      </header>

      <ToastHistory />
    </>
  );
}
