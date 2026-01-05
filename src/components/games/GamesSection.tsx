import { useEffect, useState } from 'react';
import { GameCard } from './GameCard';
import { gameVersionService, GameStatus } from '../../services/gameVersionService';
import { GameService } from '../../services/gameService';
import { useGameStore } from '../../stores/gameStore';
import { useConnectionStore } from '../../stores/connectionStore';
import { toast } from '../../lib/toast';
import { Loader2 } from 'lucide-react';
import { useTauriEvent } from '../../hooks/useTauriEvent';
import type { ArceusEvent } from '../../types/events.types';
import { DialogOverlay } from '../dialogs/DialogOverlay';
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from '../dialogs/DialogWindow';

export function GamesSection() {
  const [games, setGames] = useState<GameStatus[]>([]);
  const [loading, setLoading] = useState(true);
  const [updatingGameIds, setUpdatingGameIds] = useState<Set<number>>(new Set());
  const [showUpdateConfirmDialog, setShowUpdateConfirmDialog] = useState(false);
  const [gameToUpdate, setGameToUpdate] = useState<{ id: number; name: string; version: string } | null>(null);
  const { currentGame, setCurrentGame } = useGameStore();
  const { setIsOnline } = useConnectionStore();

  const loadGames = async () => {
    try {
      setLoading(true);
      const gameList = await gameVersionService.getGameList();
      setGames(gameList);
      // Update global connection status from first game (all will have same status)
      setIsOnline(gameList.length > 0 ? gameList[0].online : false);
    } catch (error) {
      console.error('Failed to load games:', error);
      setIsOnline(false);
      toast.error('Failed to load games', {
        description: 'Could not load games list',
      });
    } finally {
      setLoading(false);
    }
  };

  // Listen for game download progress events (page-specific state)
  useTauriEvent<ArceusEvent>('arceus://event', (event) => {
    if (event.type === 'gameDownloadProgress') {
      setGames((games) =>
        games.map((game) =>
          game.gameId === event.gameId
            ? {
                ...game,
                downloadProgress: {
                  percentage: event.percentage,
                },
              }
            : game
        )
      );

      // When download completes, clear progress and reload to get updated version
      if (event.percentage >= 100) {
        setTimeout(() => {
          setGames((games) =>
            games.map((game) =>
              game.gameId === event.gameId ? { ...game, downloadProgress: null } : game
            )
          );
          setUpdatingGameIds((prev) => {
            const next = new Set(prev);
            next.delete(event.gameId);
            return next;
          });
          loadGames();
        }, 2000);
      }
    }
  });

  useEffect(() => {
    loadGames();
  }, []);

  const handleUpdate = async (gameId: number) => {
    const game = games.find((g) => g.gameId === gameId);
    if (!game) return;

    setGameToUpdate({ id: gameId, name: game.gameName, version: game.assignedVersion });
    setShowUpdateConfirmDialog(true);
  };

  const executeUpdate = async () => {
    if (!gameToUpdate) return;

    setShowUpdateConfirmDialog(false);
    setUpdatingGameIds((prev) => new Set(prev).add(gameToUpdate.id));

    toast.info('Download Started', {
      description: `Downloading ${gameToUpdate.name}...`,
    });

    try {
      await gameVersionService.downloadGame(gameToUpdate.id);
      toast.success('Installation Complete', {
        description: `${gameToUpdate.name} has been installed successfully`,
      });
      loadGames();
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to install game';
      toast.error('Installation Failed', { description: message });
      setUpdatingGameIds((prev) => {
        const next = new Set(prev);
        next.delete(gameToUpdate.id);
        return next;
      });
    } finally {
      setGameToUpdate(null);
    }
  };

  const handleLaunch = async (_gameId: number, gameName: string) => {
    try {
      await GameService.startGame({
        name: gameName,
        exePath: `C:\\Combatica\\${gameName}\\${gameName}\\${gameName}.exe`,
        contentPath: `C:\\Combatica\\${gameName}\\ServerData`,
        packageName: `com.CombaticaLTD.${gameName}`,
      });
      setCurrentGame({ gameName });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      toast.error('Launch Failed', { description: message });
    }
  };

  const handleStop = async () => {
    if (!currentGame) return;

    try {
      await GameService.stopGame();
      const { gameName } = currentGame;
      setCurrentGame(null);
      toast.info('Game Stopped', { description: `${gameName} has been stopped` });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to stop the game';
      toast.error('Stop Failed', { description: message });
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="w-8 h-8 animate-spin text-primary-default" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Games Grid */}
      {games.length === 0 ? (
        <div className="text-center py-12 bg-grey-800 rounded-lg border border-grey-700">
          <p className="text-grey-300">No games configured</p>
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

      {/* Update Confirmation Dialog */}
      {showUpdateConfirmDialog && (
        <DialogOverlay
          onClose={() => {
            setShowUpdateConfirmDialog(false);
            setGameToUpdate(null);
          }}
        >
          <DialogWindow className="w-120">
            <DialogHeader
              title={
                gameToUpdate?.name && games.find((g) => g.gameId === gameToUpdate.id)?.installedVersion
                  ? 'Update Game'
                  : 'Install Game'
              }
            />
            <DialogContent className="pb-6">
              <p>
                Installing <span className="text-white font-medium">{gameToUpdate?.name} v{gameToUpdate?.version}</span> will require you to <span className='text-warning-default'>manually update your headsets</span>.
              </p>
            </DialogContent>
            <DialogFooter
              confirmText={
                gameToUpdate?.name && games.find((g) => g.gameId === gameToUpdate.id)?.installedVersion
                  ? 'Update'
                  : 'Install'
              }
              onConfirm={executeUpdate}
              confirmVariant="default"
              confirmDisabled={loading}
              onCancel={() => {
                setShowUpdateConfirmDialog(false);
                setGameToUpdate(null);
              }}
              cancelDisabled={loading}
            />
          </DialogWindow>
        </DialogOverlay>
      )}
    </div>
  );
}
