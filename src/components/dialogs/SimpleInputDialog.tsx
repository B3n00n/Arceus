import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Slider } from '@/components/ui/slider';
import { DialogOverlay } from './DialogOverlay';

interface SimpleInputDialogProps {
  isOpen: boolean;
  onClose: () => void;
  dialogType: 'launch-manual' | 'uninstall-manual' | 'shell' | 'volume' | 'remote-apk';
  selectedCount: number;
  onExecute: (input: string | number) => void;
  loading?: boolean;
  initialValue?: string | number;
}

export function SimpleInputDialog({
  isOpen,
  onClose,
  dialogType,
  selectedCount,
  onExecute,
  loading = false,
  initialValue,
}: SimpleInputDialogProps) {
  const [dialogInput, setDialogInput] = useState(
    typeof initialValue === 'string' ? initialValue : ''
  );
  const [volumeValue, setVolumeValue] = useState(
    typeof initialValue === 'number' ? initialValue : 50
  );

  if (!isOpen) return null;

  const handleExecute = () => {
    if (dialogType === 'volume') {
      onExecute(volumeValue);
    } else {
      onExecute(dialogInput);
    }
  };

  const getTitle = () => {
    switch (dialogType) {
      case 'launch-manual':
        return 'Launch App by Package';
      case 'uninstall-manual':
        return 'Uninstall App by Package';
      case 'shell':
        return 'Execute Shell Command';
      case 'volume':
        return 'Set Volume';
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
      case 'volume':
        return 'Volume Level (0-100)';
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
      <Card className="bg-discord-dark-2 border-discord-dark w-96">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">{getTitle()}</h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <label className="text-sm text-gray-300 mb-2 block">{getLabel()}</label>
            {dialogType === 'volume' ? (
              <div className="space-y-3">
                <Slider
                  min={0}
                  max={100}
                  value={volumeValue}
                  onValueChange={setVolumeValue}
                  className="w-full"
                />
                <div className="flex items-center justify-between">
                  <span className="text-xs text-gray-400">0</span>
                  <span className="text-lg font-semibold text-white">{volumeValue}%</span>
                  <span className="text-xs text-gray-400">100</span>
                </div>
              </div>
            ) : (
              <Input
                value={dialogInput}
                onChange={(e) => setDialogInput(e.target.value)}
                placeholder={getPlaceholder()}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') handleExecute();
                  if (e.key === 'Escape') onClose();
                }}
                autoFocus
              />
            )}
          </div>
          <div className="flex gap-2 justify-between">
            <Button
              variant="outline"
              onClick={handleExecute}
              disabled={loading || (dialogType !== 'volume' && !dialogInput.trim())}
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
