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
      className="relative rounded-lg overflow-hidden border border-grey-600 hover:border-primary-default transition-all duration-300 group h-80"
      style={backgroundStyle}
    >
      {/* Gradient Overlay - darker and blurred when running or downloading */}
      {isRunning || isDownloading ? (
        <div className="absolute inset-0 bg-black/70 backdrop-blur-xs" />
      ) : (
        <div className="absolute inset-0 bg-gradient-to-b from-black/20 to-black/80" />
      )}

      {/* Download Progress Overlay - replaces Game Details when downloading */}
      {isDownloading && game.downloadProgress ? (
        <div className="absolute bottom-0 left-0 right-0 z-20 p-6">
          <div className="w-full space-y-3">
            {/* Game Name */}
            <h3 className="text-xl font-bold text-white">
              {game.gameName}
            </h3>

            {/* Status Text */}
            <div className="flex items-center justify-between">
              <span className="text-sm text-grey-200">Downloading...</span>
              <span className="font-semibold text-yellow-500">
                {progress.toFixed(0)}%
              </span>
            </div>

            {/* Progress Bar */}
            <Progress value={progress} />
          </div>
        </div>
      ) : (
        /* Game Details Container - Bottom Left */
        <div className="absolute bottom-0 left-0 right-0 z-10 p-6 flex flex-col gap-3">
        {/* Currently Running Indicator + Game Name */}
        <div className="flex flex-col gap-1">
          {isRunning && (
            <div className="text-xs text-yellow-400 tracking-wider">
              Currently Running
            </div>
          )}

          {/* 1. Game Name */}
          <h3 className="text-xl font-bold text-white">
            {game.gameName}
          </h3>
        </div>

        {/* 2. Version + Status Badge (Horizontal) */}
        <div className="flex items-center gap-3">
          {/* Current Version or Version to Install */}
          {game.installedVersion ? (
            <span className="text-sm text-grey-200">
              v{game.installedVersion}
            </span>
          ) : game.assignedVersion ? (
            <span className="text-sm text-grey-200">
              v{game.assignedVersion}
            </span>
          ) : null}

          {/* Status Badge */}
          {!game.installedVersion ? (
            <div className="flex items-center gap-1 px-2 py-1 border-1 border-grey-500 bg-grey-800 text-white rounded text-xs font-semibold uppercase">
              Not Installed
            </div>
          ) : game.updateAvailable ? (
            <div className="flex items-center gap-1 px-2 py-1 border-1 border-warning-default bg-warning-900 text-warning-default rounded text-xs font-semibold uppercase">
              Update Available
            </div>
          ) : (
            <div className="flex items-center gap-1 px-2 py-1 border-1 border-success-default bg-success-900 text-success-default rounded text-xs font-semibold uppercase">
              Up to Date
            </div>
          )}
        </div>

        {/* 3. New Version Available Message (only show when installed and update available) */}
        {game.updateAvailable && game.assignedVersion && game.installedVersion && (
          <div className="text-sm text-grey-200">
            New version <span className="font-bold text-primary-default">v{game.assignedVersion}</span> available
          </div>
        )}

        {/* 4. Button Container */}
        <div className="flex gap-2">
          {/* Launch/Stop Button */}
          {isInstalled && (
            <div className="flex-1">
              <Button
                onClick={() => isRunning ? onStop() : onLaunch(game.gameId, game.gameName)}
                disabled={isUpdating || isDownloading}
                className="w-full"
                variant={isRunning ? 'danger-outline' : 'default'}
              >
                {isRunning ? 'Stop' : 'Launch'}
              </Button>
            </div>
          )}

          {/* Update/Install Button */}
          {game.updateAvailable && (
            <div
              className="flex-1"
              title={!game.online ? 'Offline - Not connected to server' : ''}
            >
              <Button
                onClick={() => onUpdate(game.gameId)}
                disabled={isUpdating || isRunning || !game.online}
                className="w-full"
                variant="outline_yellow"
              >
                {isDownloading ? (
                  'Downloading...'
                ) : (
                  game.installedVersion ? 'Update' : 'Install'
                )}
              </Button>
            </div>
          )}
        </div>
      </div>
      )}
    </div>
  );
}
