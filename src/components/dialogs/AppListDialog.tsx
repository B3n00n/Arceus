import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Dropdown } from '@/components/ui/dropdown';
import { DialogOverlay } from './DialogOverlay';

interface AppListDialogProps {
  isOpen: boolean;
  onClose: () => void;
  dialogType: 'launch' | 'uninstall';
  selectedCount: number;
  installedApps: string[];
  onSelectApp: (app: string) => void;
  loading?: boolean;
}

export function AppListDialog({
  isOpen,
  onClose,
  dialogType,
  selectedCount,
  installedApps,
  onSelectApp,
  loading = false,
}: AppListDialogProps) {
  const [selectedApp, setSelectedApp] = useState<string | null>(null);

  if (!isOpen) return null;

  const extractAppName = (packageName: string) => {
    const parts = packageName.split('.');
    return parts[parts.length - 1];
  };

  const getFilteredApps = () => {
    return installedApps.filter((app) => {
      // Only show CombaticaLTD apps in standard mode
      return app.startsWith('com.CombaticaLTD.');
    });
  };

  const handleSelectAndExecute = () => {
    if (selectedApp) {
      onSelectApp(selectedApp);
      handleClose();
    }
  };

  const handleClose = () => {
    setSelectedApp(null);
    onClose();
  };

  const filteredApps = getFilteredApps();

  const appDisplayNames = filteredApps.map(extractAppName);
  const displayNameToPackage = new Map(
    filteredApps.map((app) => [extractAppName(app), app])
  );
  const selectedDisplayName = selectedApp ? extractAppName(selectedApp) : null;

  return (
    <DialogOverlay onClose={handleClose}>
      <Card className="w-[500px] flex flex-col">
        <CardHeader className=''>
          <h3 className="text-lg font-semibold text-white">
            {dialogType === 'launch' ? 'Launch App' : 'Uninstall App'}
          </h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
        </CardHeader>
        <CardContent className="flex-1 p-4 pt-0">
          {filteredApps.length > 0 ? (
            <Dropdown
              options={appDisplayNames}
              value={selectedDisplayName || undefined}
              onChange={(displayName) => {
                const packageName = displayNameToPackage.get(displayName);
                if (packageName) setSelectedApp(packageName);
              }}
              placeholder="Choose an app..."
              disabled={loading}
            />
          ) : (
            <div className="text-center text-gray-400 py-3">
              {loading ? 'Loading apps...' : 'No CombaticaLTD apps found'}
            </div>
          )}
        </CardContent>
        <div className="p-4 flex-row-reverse border-t border-discord-dark flex gap-2 justify-between">
          <Button
            variant={dialogType === 'launch' ? 'default' : 'danger'}
            onClick={handleSelectAndExecute}
            disabled={loading || !selectedApp}
          >
            {dialogType === 'launch' ? 'Launch' : 'Uninstall'}
          </Button>
          <Button variant="outline" onClick={handleClose} disabled={loading}>
            Cancel
          </Button>
        </div>
      </Card>
    </DialogOverlay>
  );
}
