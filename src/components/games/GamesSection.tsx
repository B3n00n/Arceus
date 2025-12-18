import { useEffect, useState } from 'react';
import { GameCard } from './GameCard';
import { gameVersionService, GameStatus } from '../../services/gameVersionService';
import { GameService } from '../../services/gameService';
import { useGameStore } from '../../stores/gameStore';
import { toast } from '../../lib/toast';
import { Loader2, RefreshCw } from 'lucide-react';
import { Button } from '../ui/button';

export function GamesSection() {
  const [games, setGames] = useState<GameStatus[]>([]);
  const [loading, setLoading] = useState(true);
  const [updatingGameIds, setUpdatingGameIds] = useState<Set<number>>(new Set());
  const { currentGame, setCurrentGame } = useGameStore();

  const loadGames = async () => {
    try {
      setLoading(true);
      const gameList = await gameVersionService.getGameList();
      setGames(gameList);
    } catch (error) {
      console.error('Failed to load games:', error);
      toast.error('Failed to load games', {
        description: 'Could not load games list',
      });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadGames();

    // Poll for progress updates every 2 seconds when there are active downloads
    const interval = setInterval(async () => {
      if (updatingGameIds.size > 0) {
        try {
          const gameList = await gameVersionService.getGameList();
          setGames(gameList);

          // Check if any downloads completed
          const newUpdatingIds = new Set(updatingGameIds);
          for (const gameId of updatingGameIds) {
            const game = gameList.find((g) => g.gameId === gameId);
            if (game && !game.downloadProgress) {
              newUpdatingIds.delete(gameId);
            }
          }
          setUpdatingGameIds(newUpdatingIds);
        } catch (error) {
          console.error('Failed to refresh game status:', error);
        }
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [updatingGameIds]);

  const handleUpdate = async (gameId: number) => {
    try {
      setUpdatingGameIds(new Set(updatingGameIds).add(gameId));

      const game = games.find((g) => g.gameId === gameId);
      toast.info('Download Started', {
        description: `Downloading ${game?.gameName}...`,
      });

      // Start the download (non-blocking)
      gameVersionService.downloadGame(gameId).then(() => {
        toast.success('Installation Complete', {
          description: `${game?.gameName} has been installed successfully`,
        });
        loadGames();
      }).catch((error) => {
        toast.error('Installation Failed', {
          description: error.message || 'Failed to install game',
        });
        setUpdatingGameIds((prev) => {
          const newSet = new Set(prev);
          newSet.delete(gameId);
          return newSet;
        });
      });
    } catch (error) {
      console.error('Failed to start download:', error);
      toast.error('Error', {
        description: 'Failed to start game download',
      });
    }
  };

  const handleLaunch = async (gameId: number, gameName: string) => {
    try {
      const game = games.find((g) => g.gameId === gameId);
      if (!game) return;

      await GameService.startGame({
        name: gameName,
        exePath: `C:\\Combatica\\${gameName}\\${gameName}\\${gameName}.exe`,
        contentPath: `C:\\Combatica\\${gameName}\\ServerData`,
        packageName: `com.CombaticaLTD.${gameName}`,
      });

      setCurrentGame({ gameName });
    } catch (error) {
      const errorMessage = typeof error === 'string' ? error :
                          error instanceof Error ? error.message :
                          'Failed to start the game';
      toast.error('Launch Failed', {
        description: errorMessage,
      });
    }
  };

  const handleStop = async () => {
    try {
      await GameService.stopGame();
      const gameName = currentGame?.gameName;
      setCurrentGame(null);
      toast.info('Game Stopped', {
        description: `${gameName} has been stopped`,
      });
    } catch (error) {
      console.error('Failed to stop game:', error);
      toast.error('Stop Failed', {
        description: 'Failed to stop the game',
      });
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="w-8 h-8 animate-spin text-blue-500" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-white">Games</h2>
          <p className="text-gray-400 text-sm mt-1">
            Manage your game installations and updates
          </p>
        </div>
        <Button onClick={loadGames} variant="outline" size="sm">
          <RefreshCw className="w-4 h-4 mr-2" />
          Refresh
        </Button>
      </div>

      {/* Games Grid */}
      {games.length === 0 ? (
        <div className="text-center py-12 bg-gray-800/50 rounded-lg border border-gray-700">
          <p className="text-gray-400">No games configured</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {games.map((game) => (
            <GameCard
              key={game.gameId}
              game={game}
              onUpdate={handleUpdate}
              onLaunch={handleLaunch}
              onStop={handleStop}
              isUpdating={updatingGameIds.has(game.gameId)}
              isRunning={currentGame?.gameName === game.gameName}
            />
          ))}
        </div>
      )}
    </div>
  );
}
