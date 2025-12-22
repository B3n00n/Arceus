import { Download, CheckCircle, AlertCircle, Play, Square, WifiOff } from 'lucide-react';
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

  return (
    <div className="bg-gray-800 rounded-lg p-6 border border-gray-700 hover:border-blue-500 transition-colors">
      {/* Game Header */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <h3 className="text-xl font-semibold text-white mb-1">{game.gameName}</h3>
          <div className="flex items-center gap-2 text-sm">
            {game.installedVersion ? (
              <span className="text-gray-400">
                Installed: <span className="text-blue-400">{game.installedVersion}</span>
              </span>
            ) : (
              <span className="text-gray-500">Not installed</span>
            )}
          </div>
          {game.updateAvailable && game.installedVersion && (
            <div className="flex items-center gap-2 text-sm mt-1">
              <span className="text-gray-400">
                Latest: <span className="text-green-400">{game.assignedVersion}</span>
              </span>
            </div>
          )}
        </div>

        {/* Status Badge */}
        {game.updateAvailable ? (
          game.online ? (
            <div className="flex items-center gap-1 px-3 py-1 bg-yellow-500/20 text-yellow-400 rounded-full text-sm">
              <AlertCircle className="w-4 h-4" />
              Update Available
            </div>
          ) : (
            <div className="flex items-center gap-1 px-3 py-1 bg-orange-500/20 text-orange-400 rounded-full text-sm">
              <WifiOff className="w-4 h-4" />
              Update (Offline)
            </div>
          )
        ) : game.installedVersion ? (
          <div className="flex items-center gap-1 px-3 py-1 bg-green-500/20 text-green-400 rounded-full text-sm">
            <CheckCircle className="w-4 h-4" />
            Up to date
          </div>
        ) : null}
      </div>

      {/* Offline Warning Banner */}
      {game.updateAvailable && !game.online && (
        <div className="mb-4 p-3 bg-orange-500/10 border border-orange-500/30 rounded text-sm text-orange-300">
          <div className="flex items-center gap-2">
            <WifiOff className="w-4 h-4 flex-shrink-0" />
            <span>Update available but you're offline. Connect to internet to download.</span>
          </div>
        </div>
      )}

      {/* Download Progress */}
      {isDownloading && game.downloadProgress && (
        <div className="mb-4 bg-gray-900/50 rounded-lg p-4 border border-blue-500/20">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Download className="w-4 h-4 text-blue-400 animate-pulse" />
              <span className="text-sm font-medium text-blue-400">Downloading</span>
            </div>
            <span className="text-lg font-bold text-blue-400">
              {progress.toFixed(0)}%
            </span>
          </div>
          <Progress value={progress} className="h-2.5" />
        </div>
      )}

      {/* Action Buttons */}
      <div className="flex gap-2">
        {/* Launch/Stop Button (only for installed games) */}
        {isInstalled && (
          <Button
            onClick={() => isRunning ? onStop() : onLaunch(game.gameId, game.gameName)}
            disabled={isUpdating || isDownloading}
            className="flex-1"
            variant={isRunning ? 'destructive' : 'secondary'}
          >
            {isRunning ? (
              <>
                <Square className="w-4 h-4 mr-2" />
                Stop
              </>
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
            className={isInstalled ? 'flex-1' : 'w-full'}
            variant={isDownloading ? 'outline' : 'default'}
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
  );
}
