import { CheckCircle, AlertCircle, Download, WifiOff } from 'lucide-react';
import { GameStatus } from '../../services/gameVersionService';
import { Button } from '../ui/button';
import { Progress } from '../ui/progress';

interface GameCardProps {
  game: GameStatus;
  onUpdate: (gameId: number) => void;
  onLaunch: (gameId: number, gameName: string) => void;
  onStop: () => void;
  isUpdating: boolean;
  isRunning: boolean;
}

export function GameCard({ game, onUpdate, onLaunch, onStop, isUpdating, isRunning }: GameCardProps) {
  const isDownloading = game.downloadProgress !== null;
  const progress = game.downloadProgress?.percentage || 0;
  const isInstalled = game.installedVersion !== null;

  // Use local cached background image (base64 data URL) if available, otherwise black background
  const backgroundStyle = game.backgroundImagePath
    ? {
        backgroundImage: `url("${game.backgroundImagePath}")`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
      }
    : { background: '#000000' };

  return (
    <div
      className="relative rounded-lg overflow-hidden border border-gray-700 hover:border-blue-500 transition-all duration-300 group h-80"
      style={backgroundStyle}
    >
      {/* Overlay for better text readability */}
      <div className="absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-black/20" />

      {/* Game Name - Top Left */}
      <div className="absolute top-4 left-4 z-10">
        <h3 className="text-2xl font-bold text-white drop-shadow-lg">
          {game.gameName}
        </h3>
      </div>

      {/* Status Badge - Top Right */}
      <div className="absolute top-4 right-4 z-10">
        {game.updateAvailable ? (
          game.online ? (
            <div className="flex items-center gap-1 px-3 py-1 bg-yellow-500/90 text-yellow-900 rounded-full text-xs font-semibold backdrop-blur-sm">
              <AlertCircle className="w-3 h-3" />
              Update Available
            </div>
          ) : (
            <div className="flex items-center gap-1 px-3 py-1 bg-orange-500/90 text-orange-900 rounded-full text-xs font-semibold backdrop-blur-sm">
              <WifiOff className="w-3 h-3" />
              Update (Offline)
            </div>
          )
        ) : game.installedVersion ? (
          <div className="flex items-center gap-1 px-3 py-1 bg-green-500/90 text-green-900 rounded-full text-xs font-semibold backdrop-blur-sm">
            <CheckCircle className="w-3 h-3" />
            Up to date
          </div>
        ) : (
          <div className="flex items-center gap-1 px-3 py-1 bg-gray-500/90 text-gray-900 rounded-full text-xs font-semibold backdrop-blur-sm">
            Not installed
          </div>
        )}
      </div>

      {/* Download Progress - Center overlay when downloading */}
      {isDownloading && game.downloadProgress && (
        <div className="absolute inset-0 z-20 flex items-center justify-center bg-black/80 backdrop-blur-sm">
          <div className="w-3/4 space-y-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Download className="w-5 h-5 text-blue-400 animate-pulse" />
                <span className="text-sm font-medium text-blue-400">Downloading</span>
              </div>
              <span className="text-2xl font-bold text-blue-400">
                {progress.toFixed(0)}%
              </span>
            </div>
            <Progress value={progress} className="h-3" />
          </div>
        </div>
      )}

      {/* Bottom Section */}
      <div className="absolute bottom-0 left-0 right-0 z-10 p-6">
        {/* Version Info and Action Buttons */}
        <div className="flex items-end justify-between gap-4">
          {/* Version Info - Bottom Left */}
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 text-xs">
              {game.installedVersion ? (
                <span className="text-gray-300">
                  Installed: <span className="text-blue-300 font-semibold">{game.installedVersion}</span>
                </span>
              ) : null}
              {game.updateAvailable && game.installedVersion && (
                <span className="text-gray-300">
                  → Latest: <span className="text-green-300 font-semibold">{game.assignedVersion}</span>
                </span>
              )}
            </div>
          </div>

          {/* Action Buttons - Bottom Right */}
          <div className="flex gap-2 flex-shrink-0">
            {/* Launch/Stop Button (always show when installed) */}
            {isInstalled && (
              <Button
                onClick={() => isRunning ? onStop() : onLaunch(game.gameId, game.gameName)}
                disabled={isUpdating || isDownloading}
                className="shadow-lg"
                variant={isRunning ? 'destructive' : 'default'}
                size="lg"
              >
                {isRunning ? 'Stop' : 'Launch'}
              </Button>
            )}

            {/* Update/Install Button */}
            {game.updateAvailable && (
              <Button
                onClick={() => onUpdate(game.gameId)}
                disabled={isUpdating || isRunning || !game.online}
                variant={isDownloading ? 'outline' : isInstalled ? 'secondary' : 'default'}
                size="lg"
                className="shadow-lg"
              >
                {!game.online && !isDownloading ? (
                  'Offline'
                ) : isDownloading ? (
                  'Downloading...'
                ) : (
                  game.installedVersion ? 'Update' : 'Install'
                )}
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
