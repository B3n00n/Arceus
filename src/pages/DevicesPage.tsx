import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  Grid3x3,
  List,
  RefreshCw,
  CheckSquare,
  Square,
  FolderOpen,
} from 'lucide-react';
import { useDeviceStore } from '@/stores/deviceStore';
import { DeviceService } from '@/services/deviceService';
import { ApkService } from '@/services/apkService';
import { useTauriEvent } from '@/hooks/useTauriEvent';
import { cn } from '@/lib/cn';
import { toast } from '@/lib/toast';
import type { ApkInfo } from '@/types/apk.types';
import type { ArceusEvent } from '@/types/events.types';
import { DeviceList } from '@/components/devices/DeviceList';
import { CommandPanel } from '@/components/devices/CommandPanel';
import { SimpleInputDialog } from '@/components/dialogs/SimpleInputDialog';
import { AppListDialog } from '@/components/dialogs/AppListDialog';
import { ApkListDialog } from '@/components/dialogs/ApkListDialog';

export function DevicesPage() {
  const {
    viewMode,
    setViewMode,
    selectedDeviceIds,
    toggleDevice,
    selectAll,
    clearSelection,
    getFilteredDevices,
  } = useDeviceStore();

  const setDevices = useDeviceStore((state) => state.setDevices);

  const [loading, setLoading] = useState(false);

  // Dialog states
  const [showSimpleInputDialog, setShowSimpleInputDialog] = useState(false);
  const [showAppListDialog, setShowAppListDialog] = useState(false);
  const [showApkListDialog, setShowApkListDialog] = useState(false);

  const [dialogType, setDialogType] = useState<string>('');
  const [dialogInput, setDialogInput] = useState('');
  const [volumeValue] = useState(50);

  // Data states
  const [installedApps, setInstalledApps] = useState<string[]>([]);
  const [availableApks, setAvailableApks] = useState<ApkInfo[]>([]);

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    try {
      const deviceList = await DeviceService.getDevices();
      setDevices(deviceList);
    } catch (error) {
      console.error('Failed to load devices:', error);
      toast.error('Failed to load devices');
    }
  };

  const loadApks = async () => {
    try {
      const apks = await ApkService.listApks();
      setAvailableApks(apks);
    } catch (error) {
      console.error('Failed to load APKs:', error);
      toast.error('Failed to load APKs');
    }
  };

  // Listen for installedAppsReceived event (page-specific state)
  useTauriEvent<ArceusEvent>('arceus://event', (event) => {
    if (event.type === 'installedAppsReceived') {
      setInstalledApps(event.apps);
      setLoading(false);
    }
  });

  const filteredDevices = getFilteredDevices();
  const hasSelection = selectedDeviceIds.size > 0;
  const selectedIds = Array.from(selectedDeviceIds);

  const handleCommand = async (
    action: () => Promise<void>,
    actionName: string,
    showSuccessNotification: boolean = true
  ) => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }

    setLoading(true);
    try {
      await action();
      if (showSuccessNotification) {
        toast.success(`${actionName} sent to ${selectedDeviceIds.size} device(s)`);
      }
    } catch (error) {
      toast.error(`${actionName} failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };


  const openAppListDialog = async (type: 'launch' | 'uninstall') => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }

    setLoading(true);
    setInstalledApps([]);
    setDialogType(type);
    setShowAppListDialog(true);

    try {
      await DeviceService.getInstalledApps([selectedIds[0]]);
    } catch (error) {
      toast.error('Failed to request installed apps');
      setLoading(false);
    }
  };

  const openApkPickerDialog = async () => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }

    await loadApks();
    setShowApkListDialog(true);
  };

  const openSimpleInputDialog = (type: string) => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }
    setDialogType(type);
    setDialogInput('');
    setShowSimpleInputDialog(true);
  };

  const executeSimpleCommand = async (input: string | number) => {
    const inputString = typeof input === 'number' ? input.toString() : input;
    const inputNumber = typeof input === 'number' ? input : 50;

    if (dialogType !== 'volume' && !inputString.trim()) {
      toast.error('Please enter a value');
      return;
    }

    setLoading(true);
    try {
      switch (dialogType) {
        case 'launch-manual':
          await DeviceService.launchApp(selectedIds, inputString);
          break;
        case 'uninstall-manual':
          await DeviceService.uninstallApp(selectedIds, inputString);
          break;
        case 'shell':
          await DeviceService.executeShell(selectedIds, inputString);
          toast.success('Shell command sent');
          break;
        case 'volume':
          await DeviceService.setVolume(selectedIds, inputNumber);
          break;
        case 'remote-apk':
          await DeviceService.installRemoteApk(selectedIds, inputString);
          break;
      }
      setShowSimpleInputDialog(false);
      setDialogInput('');
    } catch (error) {
      toast.error(`Command failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const executeAppCommand = async (packageName: string) => {
    setLoading(true);
    try {
      if (dialogType === 'launch') {
        await DeviceService.launchApp(selectedIds, packageName);
      } else if (dialogType === 'uninstall') {
        await DeviceService.uninstallApp(selectedIds, packageName);
      }
      setShowAppListDialog(false);
    } catch (error) {
      toast.error(`Command failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const executeApkInstall = async (apk: ApkInfo) => {
    setLoading(true);
    try {
      await DeviceService.installLocalApk(selectedIds, apk.filename);
      setShowApkListDialog(false);
    } catch (error) {
      toast.error(`Install failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="h-[calc(100vh-4rem)] flex">
      {/* Left: Device List */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <div className="p-6 border-b border-discord-dark-2">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h1 className="text-2xl font-bold text-white">Devices</h1>
              <p className="text-sm text-gray-400 mt-1">
                {filteredDevices.length} device{filteredDevices.length !== 1 ? 's' : ''}
                {hasSelection && ` • ${selectedDeviceIds.size} selected`}
              </p>
            </div>
            <div className="flex gap-2">
              <Button
                onClick={async () => {
                  try {
                    await ApkService.openApkFolder();
                    toast.success('Opened APK folder');
                  } catch (error) {
                    toast.error('Failed to open folder');
                  }
                }}
                variant="outline"
                size="sm"
              >
                <FolderOpen className="h-4 w-4 mr-2" />
                APK Folder
              </Button>
              <Button onClick={loadDevices} variant="outline" size="sm" disabled={loading}>
                <RefreshCw className={cn('h-4 w-4 mr-2', loading && 'animate-spin')} />
                Refresh
              </Button>
            </div>
          </div>

          {/* Controls */}
          <div className="flex items-center gap-3">
            <div className="flex gap-1 border border-discord-dark rounded-md p-1">
              <Button
                variant={viewMode === 'grid' ? 'secondary' : 'ghost'}
                size="sm"
                onClick={() => setViewMode('grid')}
              >
                <Grid3x3 className="h-4 w-4" />
              </Button>
              <Button
                variant={viewMode === 'list' ? 'secondary' : 'ghost'}
                size="sm"
                onClick={() => setViewMode('list')}
              >
                <List className="h-4 w-4" />
              </Button>
            </div>

            <div className="flex-1" />

            <Button
              variant="outline"
              size="sm"
              onClick={selectedDeviceIds.size === filteredDevices.length ? clearSelection : selectAll}
            >
              {selectedDeviceIds.size === filteredDevices.length ? (
                <CheckSquare className="h-4 w-4 mr-2" />
              ) : (
                <Square className="h-4 w-4 mr-2" />
              )}
              Select All
            </Button>
          </div>
        </div>

        {/* Device Grid/List */}
        <div className="flex-1 overflow-y-auto p-6">
          <DeviceList
            devices={filteredDevices}
            selectedDeviceIds={selectedDeviceIds}
            viewMode={viewMode}
            onToggleDevice={toggleDevice}
          />
        </div>
      </div>

      {/* Right: Command Panel */}
      <CommandPanel
        selectedDeviceIds={selectedDeviceIds}
        loading={loading}
        onOpenAppListDialog={openAppListDialog}
        onOpenApkPickerDialog={openApkPickerDialog}
        onOpenSimpleInputDialog={openSimpleInputDialog}
        onHandleCommand={handleCommand}
      />

      {/* Dialogs */}
      <SimpleInputDialog
        isOpen={showSimpleInputDialog}
        onClose={() => setShowSimpleInputDialog(false)}
        dialogType={dialogType as 'launch-manual' | 'uninstall-manual' | 'shell' | 'volume' | 'remote-apk'}
        selectedCount={selectedDeviceIds.size}
        onExecute={executeSimpleCommand}
        loading={loading}
        initialValue={dialogType === 'volume' ? volumeValue : dialogInput}
      />

      <AppListDialog
        isOpen={showAppListDialog}
        onClose={() => setShowAppListDialog(false)}
        dialogType={dialogType as 'launch' | 'uninstall'}
        selectedCount={selectedDeviceIds.size}
        installedApps={installedApps}
        onSelectApp={executeAppCommand}
        loading={loading}
      />

      <ApkListDialog
        isOpen={showApkListDialog}
        onClose={() => setShowApkListDialog(false)}
        selectedCount={selectedDeviceIds.size}
        availableApks={availableApks}
        onSelectApk={executeApkInstall}
        loading={loading}
      />
    </div>
  );
}
