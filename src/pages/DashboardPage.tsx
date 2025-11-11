import { useState } from 'react';
import { useGameStore } from '@/stores/gameStore';
import { GameService } from '@/services/gameService';
import { toast } from '@/lib/toast';
import { Button } from '@/components/ui/button';

export function DashboardPage() {
  const [loading, setLoading] = useState(false);
  const currentGame = useGameStore((state) => state.currentGame);

  // Hardcoded game config - replace with your game paths
  const GAME_CONFIG = {
    name: 'Test Unity Game',
    exePath: 'C:\\Combatica\\Defense\\Combatica_Defense\\Combatica Platform.exe',
    contentPath: 'C:\\Combatica\\Defense\\ServerData',
    packageName: 'com.CombaticaLTD.CombaticaPlatform',
  };

  const handleToggleGame = async () => {
    try {
      setLoading(true);
      if (currentGame) {
        await GameService.stopGame();
      } else {
        await GameService.startGame(GAME_CONFIG);
      }
    } catch (error) {
      toast.error(`${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="relative min-h-[calc(100vh-4rem)] p-6">
      <div>
        <h1 className="text-3xl font-bold text-white">Dashboard</h1>
        <p className="text-gray-400 mt-1">Quest Device Manager</p>
      </div>

      {/* Button fixed at bottom right */}
      <div className="fixed bottom-6 right-6">
        <Button
          onClick={handleToggleGame}
          disabled={loading}
          variant={currentGame ? 'destructive' : 'default'}
        >
          {currentGame ? 'Stop' : 'Start'}
        </Button>
      </div>
    </div>
  );
}
