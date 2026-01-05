import { useState } from 'react';
import { Dropdown } from '@/components/ui/dropdown';
import { DialogOverlay } from './DialogOverlay';
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from './DialogWindow';

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
      <DialogWindow className="w-[500px]">
        <DialogHeader
          title={dialogType === 'launch' ? 'Launch App' : 'Uninstall App'}
          subtitle={`For ${selectedCount} device(s)`}
        />
        <DialogContent>
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
        </DialogContent>
        <DialogFooter
          confirmText={dialogType === 'launch' ? 'Launch' : 'Uninstall'}
          onConfirm={handleSelectAndExecute}
          confirmVariant={dialogType === 'launch' ? 'default' : 'danger'}
          confirmDisabled={loading || !selectedApp}
          onCancel={handleClose}
          cancelDisabled={loading}
        />
      </DialogWindow>
    </DialogOverlay>
  );
}
