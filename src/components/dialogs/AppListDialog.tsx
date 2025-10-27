import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Search } from 'lucide-react';
import { cn } from '@/lib/cn';
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
  const [dialogSearch, setDialogSearch] = useState('');
  const [selectedApp, setSelectedApp] = useState<string | null>(null);

  if (!isOpen) return null;

  const extractAppName = (packageName: string) => {
    const parts = packageName.split('.');
    return parts[parts.length - 1];
  };

  const getFilteredApps = () => {
    return installedApps.filter((app) => {
      // Only show CombaticaLTD apps in standard mode
      if (!app.startsWith('com.CombaticaLTD.')) return false;

      if (dialogSearch) {
        const appName = extractAppName(app);
        return (
          appName.toLowerCase().includes(dialogSearch.toLowerCase()) ||
          app.toLowerCase().includes(dialogSearch.toLowerCase())
        );
      }
      return true;
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
    setDialogSearch('');
    onClose();
  };

  const filteredApps = getFilteredApps();

  return (
    <DialogOverlay onClose={handleClose}>
      <Card className="bg-discord-dark-2 border-discord-dark w-[500px] max-h-[600px] flex flex-col">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">
            {dialogType === 'launch' ? 'Select App to Launch' : 'Select App to Uninstall'}
          </h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
          <div className="relative mt-2">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
            <Input
              value={dialogSearch}
              onChange={(e) => setDialogSearch(e.target.value)}
              placeholder="Search apps..."
              className="pl-10"
            />
          </div>
        </CardHeader>
        <CardContent className="flex-1 overflow-y-auto p-0">
          <div className="divide-y divide-discord-dark">
            {filteredApps.map((app) => (
              <button
                key={app}
                className={cn(
                  'w-full px-6 py-3 text-left hover:bg-discord-dark-3 transition-colors',
                  selectedApp === app && 'bg-discord-blurple/20 border-l-2 border-discord-blurple'
                )}
                onClick={() => setSelectedApp(app)}
                disabled={loading}
              >
                <p className="text-sm text-white font-medium">{extractAppName(app)}</p>
                <p className="text-xs text-gray-400 mt-0.5 font-mono">{app}</p>
              </button>
            ))}
            {filteredApps.length === 0 && (
              <div className="px-6 py-8 text-center text-gray-400">
                {loading
                  ? 'Loading apps...'
                  : installedApps.length === 0
                  ? 'No apps found'
                  : 'No CombaticaLTD apps found'}
              </div>
            )}
          </div>
        </CardContent>
        <div className="p-4 border-t border-discord-dark flex gap-2 justify-between">
          <Button
            variant="outline"
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
