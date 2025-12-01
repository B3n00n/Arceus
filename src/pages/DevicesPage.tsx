import { useEffect, useState, useMemo } from 'react';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import {
  RefreshCw,
  FolderOpen,
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
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
import { ConfirmationDialog } from '@/components/dialogs/ConfirmationDialog';
import { MessageDialog } from '@/components/dialogs/MessageDialog';

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

  // Sort states
  type SortField = 'name' | 'volume' | 'battery' | 'ip' | 'runningApp';
  type SortDirection = 'asc' | 'desc' | null;
  const [sortField, setSortField] = useState<SortField | null>(null);
  const [sortDirection, setSortDirection] = useState<SortDirection>(null);

  // Dialog states
  const [showSimpleInputDialog, setShowSimpleInputDialog] = useState(false);
  const [showAppListDialog, setShowAppListDialog] = useState(false);
  const [showInstallApkDialog, setShowInstallApkDialog] = useState(false);
  const [showRestartDeviceDialog, setShowRestartDeviceDialog] = useState(false);
  const [showCloseAllAppsDialog, setShowCloseAllAppsDialog] = useState(false);
  const [showMessageDialog, setShowMessageDialog] = useState(false);

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

  // Sort devices
  const sortedDevices = useMemo(() => {
    if (!sortField || !sortDirection) return filteredDevices;

    return [...filteredDevices].sort((a, b) => {
      let aValue: string | number | null = null;
      let bValue: string | number | null = null;

      switch (sortField) {
        case 'name':
          aValue = (a.info.customName || a.info.model || '').toLowerCase();
          bValue = (b.info.customName || b.info.model || '').toLowerCase();
          break;
        case 'volume':
          aValue = a.volume?.volumePercentage ?? -1;
          bValue = b.volume?.volumePercentage ?? -1;
          break;
        case 'battery':
          aValue = a.battery?.headsetLevel ?? -1;
          bValue = b.battery?.headsetLevel ?? -1;
          break;
        case 'ip':
          aValue = a.info.ip;
          bValue = b.info.ip;
          break;
        case 'runningApp':
          aValue = (a.info.runningApp || '').toLowerCase();
          bValue = (b.info.runningApp || '').toLowerCase();
          break;
      }

      if (aValue === null || aValue === -1) return 1;
      if (bValue === null || bValue === -1) return -1;

      if (typeof aValue === 'string' && typeof bValue === 'string') {
        return sortDirection === 'asc'
          ? aValue.localeCompare(bValue)
          : bValue.localeCompare(aValue);
      }

      return sortDirection === 'asc'
        ? (aValue as number) - (bValue as number)
        : (bValue as number) - (aValue as number);
    });
  }, [filteredDevices, sortField, sortDirection]);

  const hasSelection = selectedDeviceIds.size > 0;
  const selectedIds = Array.from(selectedDeviceIds);
  const allSelected =
    sortedDevices.length > 0 &&
    selectedDeviceIds.size === sortedDevices.length;

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

  const openRestartDeviceDialog = () => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }
    setShowRestartDeviceDialog(true);
  };

  const openCloseAllAppsDialog = () => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }
    setShowCloseAllAppsDialog(true);
  };

  const openMessageDialog = () => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }
    setShowMessageDialog(true);
  };

  const executeRestartDevice = async () => {
    await handleCommand(
      () => DeviceService.restartDevices(selectedIds),
      'Restart',
      false
    );
    setShowRestartDeviceDialog(false);
  };

  const executeCloseAllApps = async () => {
    await handleCommand(
      () => DeviceService.closeAllApps(selectedIds),
      'Close All Apps'
    );
    setShowCloseAllAppsDialog(false);
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

  const executeSendMessage = async (message: string) => {
    await handleCommand(
      () => DeviceService.displayMessage(selectedIds, message),
      'Send Message'
    );
    setShowMessageDialog(false);
  };

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      // Cycle through: asc -> desc -> null
      if (sortDirection === 'asc') {
        setSortDirection('desc');
      } else if (sortDirection === 'desc') {
        setSortField(null);
        setSortDirection(null);
      }
    } else {
      setSortField(field);
      setSortDirection('asc');
    }
  };

  const getSortIcon = (field: SortField) => {
    if (sortField !== field) {
      return <ArrowUpDown className="h-3.5 w-3.5 text-gray-500" />;
    }
    return sortDirection === 'asc'
      ? <ArrowUp className="h-3.5 w-3.5 text-blue-400" />
      : <ArrowDown className="h-3.5 w-3.5 text-blue-400" />;
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
                {sortedDevices.length} device{sortedDevices.length !== 1 ? 's' : ''}
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
          {sortedDevices.length > 0 && (
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
              <button
                onClick={() => handleSort('name')}
                className="flex-[2] min-w-[8rem] flex justify-start items-center gap-2 hover:text-gray-300 transition-colors"
              >
                <span>Device</span>
                {getSortIcon('name')}
              </button>

      {/* Running App */}
      <button
        onClick={() => handleSort('runningApp')}
        className="flex-[1.5] min-w-[8rem] flex justify-start items-center gap-2 hover:text-gray-300 transition-colors"
      >
        <span>Running App</span>
        {getSortIcon('runningApp')}
      </button>

              {/* IP */}
              <button
                onClick={() => handleSort('ip')}
                className="flex-[1.5] min-w-[8rem] flex justify-start items-center gap-2 hover:text-gray-300 transition-colors"
              >
                <span>IP</span>
                {getSortIcon('ip')}
              </button>

              {/* Volume */}
              <button
                onClick={() => handleSort('volume')}
                className="flex-[0.75] min-w-[4rem] flex justify-start items-center gap-2 hover:text-gray-300 transition-colors"
              >
                <span>Volume</span>
                {getSortIcon('volume')}
              </button>

              {/* Battery */}
              <button
                onClick={() => handleSort('battery')}
                className="flex-[0.75] min-w-[4rem] flex justify-start items-center gap-2 hover:text-gray-300 transition-colors"
              >
                <span>Battery</span>
                {getSortIcon('battery')}
              </button>
            </div>
          )}

          {/* Device List */}
          <DeviceList
            devices={sortedDevices}
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
        onOpenRestartDeviceDialog={openRestartDeviceDialog}
        onOpenCloseAllAppsDialog={openCloseAllAppsDialog}
        onOpenMessageDialog={openMessageDialog}
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

      <ConfirmationDialog
        isOpen={showRestartDeviceDialog}
        onClose={() => setShowRestartDeviceDialog(false)}
        onConfirm={executeRestartDevice}
        title="Restart Device"
        message={
          <>
            Restart{" "}
            <span className="text-white font-medium">
              {selectedDeviceIds.size} device{selectedDeviceIds.size > 1 ? "s" : ""}
            </span>
            ?
          </>
        }
        confirmText="Restart"
        loading={loading}
      />

      <ConfirmationDialog
        isOpen={showCloseAllAppsDialog}
        onClose={() => setShowCloseAllAppsDialog(false)}
        onConfirm={executeCloseAllApps}
        title="Close All Apps"
        message={
          <>
            Close all running apps on{" "}
            <span className="text-white font-medium">
              {selectedDeviceIds.size} device{selectedDeviceIds.size > 1 ? "s" : ""}
            </span>
            ?
          </>
        }
        confirmText="Close"
        loading={loading}
      />

      <MessageDialog
        isOpen={showMessageDialog}
        onClose={() => setShowMessageDialog(false)}
        selectedCount={selectedDeviceIds.size}
        onExecute={executeSendMessage}
        loading={loading}
      />
    </div>
  );
}
