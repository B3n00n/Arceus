import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DialogOverlay } from "./DialogOverlay";
import { GameService } from "@/services/gameService";
import { useGameStore } from "@/stores/gameStore";

interface StopAppDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function StopAppDialog({ isOpen, onClose }: StopAppDialogProps) {
  const { currentGame, setCurrentGame } = useGameStore();

  if (!isOpen) return null;

  const handleStop = async () => {
    try {
      await GameService.stopGame();
      setCurrentGame(null);
      onClose();
    } catch (error) {
    }
  };

  return (
    <DialogOverlay onClose={onClose}>
      <Card className="w-100">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">Stop Running App</h3>
        </CardHeader>

        <CardContent className="space-y-8">
          <div className="text-md text-gray-300">
            Are you sure you want to stop{" "}
            <span className="text-white font-medium">
              {currentGame?.gameName || "the current app"}
            </span>
            ?
          </div>

          {/* Actions */}
          <div className="flex flex-row-reverse justify-between gap-2 pt-1">
            <Button variant="danger" onClick={handleStop}>
              Stop App
            </Button>
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
          </div>
        </CardContent>
      </Card>
    </DialogOverlay>
  );
}
