import { Bell } from "lucide-react";
import { useLocation } from "react-router-dom";
import { useToastHistoryStore } from "@/stores/toastHistoryStore";
import { ToastHistory } from "@/components/ToastHistory";

export function Header() {
  const { togglePanel, unreadCount } = useToastHistoryStore();
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

  return (
    <>
      <header className="sticky top-0 z-30 h-16 border-b border-discord-dark-2 bg-dark">
        <div className="flex h-full items-center justify-between px-6">
          {/* Left — Title */}
          <div className="flex items-center gap-2">
            <h1 className="text-base font-semibold text-white text-lg">{title}</h1>
          </div>

          {/* Right — Controls */}
          <div className="flex items-center gap-3">
            {/* Notifications */}
            <button
              onClick={togglePanel}
              className="relative p-2 rounded-lg hover:bg-discord-dark-2 transition-colors group"
            >
              <Bell className="h-5 w-5 text-gray-400 group-hover:text-white transition-colors" />
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
