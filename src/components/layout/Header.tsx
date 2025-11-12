import { useState } from "react";
import { Bell, Rocket, Square } from "lucide-react";
import { useLocation } from "react-router-dom";
import { useToastHistoryStore } from "@/stores/toastHistoryStore";
import { useGameStore } from "@/stores/gameStore";
import { GameService } from "@/services/gameService";
import { toast } from "@/lib/toast";
import { Button } from "@/components/ui/button";
import { ToastHistory } from "@/components/ToastHistory";
import { LaunchAppDialog } from "@/components/dialogs/LaunchAppDialog";
import { StopAppDialog } from "@/components/dialogs/StopAppDialog";

export function Header() {
  const { togglePanel, unreadCount } = useToastHistoryStore();
  const { currentGame, setCurrentGame } = useGameStore();
  const location = useLocation();

  const [loading, setLoading] = useState(false);
  const [showLaunchDialog, setShowLaunchDialog] = useState(false);
  const [showStopDialog, setShowStopDialog] = useState(false);

  // Example available apps (replace with actual list from backend)
  const availableApps = [
    { name: "Combatica Platform", packageName: "com.CombaticaLTD.CombaticaPlatform" },
    { name: "Sample VR Training", packageName: "com.Example.SampleVR" },
    { name: "Arena Shooter", packageName: "com.Example.ArenaShooter" },
  ];

  const handleLaunch = async (
    app: { name: string; packageName: string },
    launchOnClients: boolean
  ) => {
    try {
      setLoading(true);
      await GameService.startGame({
        name: app.name,
        exePath: "C:\\Combatica\\Defense\\Combatica_Defense\\Combatica Platform.exe",
        contentPath: "C:\\Combatica\\Defense\\ServerData",
        packageName: app.packageName,
      });
      setCurrentGame({ gameName: app.name });
      toast.success(`Launched ${app.name}${launchOnClients ? " on clients" : ""}`);
      setShowLaunchDialog(false);
    } catch (error) {
      toast.error(`Failed to launch: ${error}`);
    } finally {
      setLoading(false);
    }
  };

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
            {/* Game Running Status */}
            {currentGame && (
              <div className="px-2 py-1.5">
                <span className="text-xs text-gray-300 font-regular">
                  Running:{" "}
                  <span className="text-xs text-white font-medium">
                    {currentGame.gameName}
                  </span>
                </span>
              </div>
            )}

            {/* Launch / Stop Buttons */}
            {currentGame ? (
              <Button
                size="sm"
                variant="danger-outline"
                onClick={() => setShowStopDialog(true)}
                disabled={loading}
                className="flex items-center gap-2"
              >
                <Square className="h-4 w-4" />
                Stop
              </Button>
            ) : (
              <Button
                size="sm"
                variant="default"
                onClick={() => setShowLaunchDialog(true)}
                disabled={loading}
                className="flex items-center gap-2"
              >
                <Rocket className="h-4 w-4" />
                Launch App
              </Button>
            )}

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

      {/* Dialogs */}
      <LaunchAppDialog
        isOpen={showLaunchDialog}
        onClose={() => setShowLaunchDialog(false)}
        availableApps={availableApps}
        onLaunch={handleLaunch}
      />

      <StopAppDialog
        isOpen={showStopDialog}
        onClose={() => setShowStopDialog(false)}
      />

      <ToastHistory />
    </>
  );
}
