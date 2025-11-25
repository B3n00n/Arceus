import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DialogOverlay } from "./DialogOverlay";

interface CloseAllAppsDialogProps {
  isOpen: boolean;
  onClose: () => void;
  deviceCount: number;
  onConfirm: () => void;
  loading?: boolean;
}

export function CloseAllAppsDialog({
  isOpen,
  onClose,
  deviceCount,
  onConfirm,
  loading = false,
}: CloseAllAppsDialogProps) {
  if (!isOpen) return null;

  return (
    <DialogOverlay onClose={onClose}>
      <Card className="w-100">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">Close All Apps</h3>
        </CardHeader>

        <CardContent className="space-y-8">
          <div className="text-md text-gray-300">
            Close all running apps on{" "}
            <span className="text-white font-medium">
              {deviceCount} device{deviceCount > 1 ? "s" : ""}
            </span>
            ?
          </div>
 </CardContent>
          {/* Actions */}
          <div className="p-4 border-t border-discord-dark flex flex-row-reverse justify-between gap-2">
            <Button variant="danger" onClick={onConfirm} disabled={loading}>
              Close
            </Button>
            <Button variant="outline" onClick={onClose} disabled={loading}>
              Cancel
            </Button>
          </div>
       
      </Card>
    </DialogOverlay>
  );
}
