import { Bell, Wifi, WifiOff, RefreshCw } from "lucide-react";
import { useLocation } from "react-router-dom";
import { useToastHistoryStore } from "@/stores/toastHistoryStore";
import { useConnectionStore } from "@/stores/connectionStore";
import { ToastHistory } from "@/components/ToastHistory";
import { gameVersionService } from "@/services/gameVersionService";
import { toast } from "@/lib/toast";
import { useState } from "react";

export function Header() {
  const { togglePanel, unreadCount } = useToastHistoryStore();
  const { isOnline, setIsOnline } = useConnectionStore();
  const [isRefreshing, setIsRefreshing] = useState(false);
  const location = useLocation();

  const pathname = location.pathname || "/";
  const title = (() => {
    if (pathname === "/" || pathname === "") return "Dashboard";
    if (pathname.startsWith("/devices")) return "Devices";
    if (pathname.startsWith("/apk-manager")) return "APK Manager";
    if (pathname.startsWith("/settings")) return "Settings";
    const last = pathname.split("/").filter(Boolean).pop();
    return last
      ? last.replace(/[-_]/g, " ").replace(/\b\w/g, (s) => s.toUpperCase())
      : "App";
  })();

  const handleRefresh = async () => {
    setIsRefreshing(true);
    try {
      const games = await gameVersionService.forceRefresh();
      setIsOnline(games.length > 0 ? games[0].online : false);
      toast.success('Connection Status Update', {
        description: games[0]?.online ? 'Connected to server' : 'Failed to connect to server',
      });
    } catch (error) {
      setIsOnline(false);
      toast.error('Connection Failed', {
        description: 'Unable to reach server',
      });
    } finally {
      setIsRefreshing(false);
    }
  };

  return (
    <>
      <header className="sticky top-0 z-30 h-16 border-b border-grey-700 bg-grey-900">
        <div className="flex h-full items-center justify-between px-6">
          {/* Left — Title */}
          <div className="flex items-center gap-2">
            <h1 className="text-base font-semibold text-white text-lg">{title}</h1>
          </div>

          {/* Right — Controls */}
          <div className="flex items-center gap-2">
            {/* Refresh Connection Button */}
            <button
              onClick={handleRefresh}
              disabled={isRefreshing}
              className="p-2 rounded-lg hover:bg-grey-700 transition-colors group disabled:opacity-50 disabled:cursor-not-allowed"
              title="Refresh connection to server"
            >
              <RefreshCw className={`h-5 w-5 text-grey-300 group-hover:text-white transition-colors ${isRefreshing ? 'animate-spin' : ''}`} />
            </button>

            {/* WiFi Status Indicator */}
            <div
              className="p-2 rounded-lg"
              title={isOnline ? 'Connected to server' : 'Offline - Not connected to server'}
            >
              {isOnline ? (
                <Wifi className="h-5 w-5 text-success-default" />
              ) : (
                <WifiOff className="h-5 w-5 text-warning-default" />
              )}
            </div>

            {/* Notifications */}
            <button
              onClick={togglePanel}
              className="relative p-2 rounded-lg hover:bg-grey-700 transition-colors group"
              title="Notifications"
            >
              <Bell className="h-5 w-5 text-grey-300 group-hover:text-white transition-colors" />
              {unreadCount > 0 && (
                <span className="absolute -top-1 -right-1 h-5 w-5 bg-red-500 rounded-full flex items-center justify-center">
                  <span className="text-xs font-semibold text-white">
                    {unreadCount > 9 ? "9+" : unreadCount}
                  </span>
                </span>
              )}
            </button>
          </div>
        </div>
      </header>

      <ToastHistory />
    </>
  );
}
