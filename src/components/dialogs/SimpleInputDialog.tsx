import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { DialogOverlay } from './DialogOverlay';

interface SimpleInputDialogProps {
  isOpen: boolean;
  onClose: () => void;
  dialogType: 'launch-manual' | 'uninstall-manual' | 'shell' | 'remote-apk';
  selectedCount: number;
  onExecute: (input: string) => void;
  loading?: boolean;
  initialValue?: string;
}

export function SimpleInputDialog({
  isOpen,
  onClose,
  dialogType,
  selectedCount,
  onExecute,
  loading = false,
  initialValue = '',
}: SimpleInputDialogProps) {
  const [dialogInput, setDialogInput] = useState(initialValue);

  if (!isOpen) return null;

  const handleExecute = () => {
    if (!dialogInput.trim()) return;
    onExecute(dialogInput);
  };

  const getTitle = () => {
    switch (dialogType) {
      case 'launch-manual':
        return 'Launch App by Package';
      case 'uninstall-manual':
        return 'Uninstall App by Package';
      case 'shell':
        return 'Execute Shell Command';
      case 'remote-apk':
        return 'Install APK from URL';
    }
  };

  const getLabel = () => {
    switch (dialogType) {
      case 'launch-manual':
      case 'uninstall-manual':
        return 'Package Name';
      case 'shell':
        return 'Shell Command';
      case 'remote-apk':
        return 'APK URL';
    }
  };

  const getPlaceholder = () => {
    switch (dialogType) {
      case 'launch-manual':
      case 'uninstall-manual':
        return 'com.example.app';
      case 'shell':
        return 'ls -la';
      case 'remote-apk':
        return 'https://example.com/app.apk';
      default:
        return '';
    }
  };

  return (
    <DialogOverlay onClose={onClose}>
      <Card className="w-96">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">{getTitle()}</h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
        </CardHeader>

        <CardContent className="space-y-4">
          <div>
            <label className="text-sm text-gray-300 mb-2 block">{getLabel()}</label>
            <Input
              value={dialogInput}
              onChange={(e) => setDialogInput(e.target.value)}
              placeholder={getPlaceholder()}
              onKeyDown={(e) => {
                if (e.key === 'Enter') handleExecute();
                if (e.key === 'Escape') onClose();
              }}
              autoFocus
              disabled={loading}
            />
          </div>

          <div className="flex gap-2 flex-row-reverse justify-between">
            <Button
              variant="default"
              onClick={handleExecute}
              disabled={loading || !dialogInput.trim()}
            >
              Execute
            </Button>
            <Button variant="outline" onClick={onClose} disabled={loading}>
              Cancel
            </Button>
          </div>
        </CardContent>
      </Card>
    </DialogOverlay>
  );
}
