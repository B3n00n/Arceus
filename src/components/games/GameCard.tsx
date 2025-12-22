import { Download, CheckCircle, AlertCircle, Play, WifiOff } from 'lucide-react';
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

// TODO: Remove this and use a proper image.
function getGradientFromName(name: string): string {
  const hash = name.split('').reduce((acc, char) => {
    return char.charCodeAt(0) + ((acc << 5) - acc);
  }, 0);

  const hue1 = Math.abs(hash % 360);
  const hue2 = Math.abs((hash * 2) % 360);

  return `linear-gradient(135deg, hsl(${hue1}, 70%, 35%) 0%, hsl(${hue2}, 60%, 25%) 100%)`;
}

export function GameCard({ game, onUpdate, onLaunch, onStop, isUpdating, isRunning }: GameCardProps) {
  const isDownloading = game.downloadProgress !== null;
  const progress = game.downloadProgress?.percentage || 0;
  const isInstalled = game.installedVersion !== null;
  const gradient = getGradientFromName(game.gameName);

  return (
    <div
      className="relative rounded-lg overflow-hidden border border-gray-700 hover:border-blue-500 transition-all duration-300 group h-80"
      style={{ background: gradient }}
    >
      {/* Overlay for better text readability */}
      <div className="absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-black/20" />

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
        {/* Version Info */}
        <div className="mb-3 space-y-1">
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

        {/* Game Name and Action Button */}
        <div className="flex items-end justify-between gap-4">
          {/* Game Name - Bottom Left */}
          <div className="flex-1 min-w-0">
            <h3 className="text-2xl font-bold text-white truncate drop-shadow-lg">
              {game.gameName}
            </h3>
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
                {isRunning ? (
                  'Stop'
                ) : (
                  <>
                    <Play className="w-4 h-4 mr-2" />
                    Launch
                  </>
                )}
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
                  <>
                    <WifiOff className="w-4 h-4 mr-2" />
                    Offline
                  </>
                ) : isDownloading ? (
                  <>
                    <Download className="w-4 h-4 mr-2 animate-pulse" />
                    Downloading...
                  </>
                ) : (
                  <>
                    <Download className="w-4 h-4 mr-2" />
                    {game.installedVersion ? 'Update' : 'Install'}
                  </>
                )}
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
