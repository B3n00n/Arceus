import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import {
  RefreshCw,
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
import { InstallApkDialog } from '@/components/dialogs/InstallApkDialog';

export function DevicesPage() {
  const {
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
  const [showInstallApkDialog, setShowInstallApkDialog] = useState(false);

  const [dialogType, setDialogType] = useState<string>('');
  const [dialogInput, setDialogInput] = useState('');

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
  const allSelected =
    filteredDevices.length > 0 &&
    selectedDeviceIds.size === filteredDevices.length;

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

  const openInstallApkDialog = async () => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }

    await loadApks();
    setShowInstallApkDialog(true);
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

  const executeLocalApkInstall = async (apk: ApkInfo) => {
    setLoading(true);
    try {
      await DeviceService.installLocalApk(selectedIds, apk.filename);
      setShowInstallApkDialog(false);
      toast.success(`Installing ${apk.filename} on ${selectedIds.length} device(s)`);
    } catch (error) {
      toast.error(`Install failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const executeRemoteApkInstall = async (url: string) => {
    setLoading(true);
    try {
      await DeviceService.installRemoteApk(selectedIds, url);
      setShowInstallApkDialog(false);
      toast.success(`Installing APK from URL on ${selectedIds.length} device(s)`);
    } catch (error) {
      toast.error(`Install failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="h-[calc(100vh-4rem)] flex">
      {/* Left: Device List */}
      <div className="flex-1 flex flex-col overflow-hidden p-6">
        {/* Header */}
        <div className="pl-4">
          <div className="flex items-center justify-between mb-4">
            <div>
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
                <FolderOpen className="h-4 w-4" />
                APK Folder
              </Button>
              <Button onClick={loadDevices} variant="outline" size="sm" disabled={loading}>
                <RefreshCw className={cn('h-4 w-4', loading && 'animate-spin')} />
                Refresh
              </Button>
            </div>
          </div>

        </div>

        {/* Device Scrollable Area */}
        <div className="flex-1 overflow-y-auto overflow-x-auto space-y-2">
          {filteredDevices.length > 0 && (
            <div
              className="
        p-4 text-gray-400 text-sm
        min-w-fit flex items-center w-full gap-8 outline-offset-[-1px]
      "
            >
              {/* Checkbox for Select All */}
              <div className="flex-shrink-0 flex items-center justify-start">
                <Checkbox
                  checked={allSelected}
                  onCheckedChange={() =>
                    allSelected ? clearSelection() : selectAll()
                  }
                  className="border-discord-dark-3"
                />
              </div>

              {/* Device */}
              <div className="flex-[2] min-w-[8rem] flex justify-start items-center">
                <span>Device</span>
              </div>

              {/* MAC */}
              <div className="flex-[1.5] min-w-[8rem] flex justify-start items-center">
                <span>MAC</span>
              </div>

              {/* IP */}
              <div className="flex-[1.5] min-w-[8rem] flex justify-start items-center">
                <span>IP</span>
              </div>

              {/* Volume */}
              <div className="flex-[0.75] min-w-[4rem] flex justify-start items-center">
                <span>Volume</span>
              </div>

              {/* Battery */}
              <div className="flex-[0.75] min-w-[4rem] flex justify-start items-center">
                <span>Battery</span>
              </div>
            </div>
          )}

          {/* Device List */}
          <DeviceList
            devices={filteredDevices}
            selectedDeviceIds={selectedDeviceIds}
            onToggleDevice={toggleDevice}
          />
        </div>

      </div>

      {/* Right: Command Panel */}
      <CommandPanel
        selectedDeviceIds={selectedDeviceIds}
        loading={loading}
        onOpenAppListDialog={openAppListDialog}
        onOpenInstallApkDialog={openInstallApkDialog}
        onOpenSimpleInputDialog={openSimpleInputDialog}
        onHandleCommand={handleCommand}
      />

      {/* Dialogs */}
      <SimpleInputDialog
        isOpen={showSimpleInputDialog}
        onClose={() => setShowSimpleInputDialog(false)}
        dialogType={dialogType as 'launch-manual' | 'uninstall-manual' | 'shell' | 'remote-apk'}
        selectedCount={selectedDeviceIds.size}
        onExecute={executeSimpleCommand}
        loading={loading}
        initialValue={dialogInput}
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

      <InstallApkDialog
        isOpen={showInstallApkDialog}
        onClose={() => setShowInstallApkDialog(false)}
        selectedCount={selectedDeviceIds.size}
        availableApks={availableApks}
        onInstallLocal={executeLocalApkInstall}
        onInstallRemote={executeRemoteApkInstall}
        loading={loading}
      />
    </div>
  );
}
