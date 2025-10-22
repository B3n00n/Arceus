import { useEffect, useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import {
  Grid3x3,
  List,
  Battery,
  Wifi,
  Volume2,
  RefreshCw,
  Power,
  CheckSquare,
  Square,
  Trash2,
  PlayCircle,
  Terminal,
  Download,
  Upload,
  Search,
  X,
  FileText,
  FolderOpen,
  Pencil,
  Check,
} from 'lucide-react';
import { useDeviceStore } from '@/stores/deviceStore';
import { DeviceService } from '@/services/deviceService';
import { ApkService } from '@/services/apkService';
import { useTauriEvent } from '@/hooks/useTauriEvent';
import { formatDate, getBatteryColor, getStatusColor } from '@/lib/formatting';
import { cn } from '@/lib/cn';
import { toast } from 'sonner';
import type { DeviceState } from '@/types/device.types';
import type { ApkInfo } from '@/types/apk.types';
import type { ArceusEvent } from '@/types/events.types';

type CommandTab = 'standard' | 'dev';

export function DevicesPage() {
  const {
    viewMode,
    setViewMode,
    filterStatus,
    setFilterStatus,
    selectedDeviceIds,
    toggleDevice,
    selectAll,
    clearSelection,
    getFilteredDevices,
  } = useDeviceStore();

  const setDevices = useDeviceStore((state) => state.setDevices);

  const [loading, setLoading] = useState(false);
  const [commandTab, setCommandTab] = useState<CommandTab>('standard');

  // Dialog states
  const [showSimpleInputDialog, setShowSimpleInputDialog] = useState(false);
  const [showAppListDialog, setShowAppListDialog] = useState(false);
  const [showApkListDialog, setShowApkListDialog] = useState(false);

  const [dialogType, setDialogType] = useState<string>('');
  const [dialogInput, setDialogInput] = useState('');
  const [dialogSearch, setDialogSearch] = useState('');

  // Selection states for dialogs
  const [selectedApp, setSelectedApp] = useState<string | null>(null);
  const [selectedApk, setSelectedApk] = useState<ApkInfo | null>(null);

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
    setDialogSearch('');
    setSelectedApp(null);
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
    setDialogSearch('');
    setSelectedApk(null);
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

  const executeSimpleCommand = async () => {
    if (!dialogInput.trim()) {
      toast.error('Please enter a value');
      return;
    }

    setLoading(true);
    try {
      switch (dialogType) {
        case 'launch-manual':
          await DeviceService.launchApp(selectedIds, dialogInput);
          break;
        case 'uninstall-manual':
          await DeviceService.uninstallApp(selectedIds, dialogInput);
          break;
        case 'shell':
          await DeviceService.executeShell(selectedIds, dialogInput);
          toast.success('Shell command sent');
          break;
        case 'volume':
          const vol = parseInt(dialogInput);
          if (isNaN(vol) || vol < 0 || vol > 100) {
            toast.error('Volume must be 0-100');
            return;
          }
          await DeviceService.setVolume(selectedIds, vol);
          break;
        case 'remote-apk':
          await DeviceService.installRemoteApk(selectedIds, dialogInput);
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
      setSelectedApp(null);
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
      setSelectedApk(null);
    } catch (error) {
      toast.error(`Install failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const extractAppName = (packageName: string) => {
    const parts = packageName.split('.');
    return parts[parts.length - 1];
  };

  const getFilteredApps = () => {
    return installedApps.filter(app => {
      // Only show CombaticaLTD apps in standard mode
      if (!app.startsWith('com.CombaticaLTD.')) return false;

      if (dialogSearch) {
        const appName = extractAppName(app);
        return appName.toLowerCase().includes(dialogSearch.toLowerCase()) ||
               app.toLowerCase().includes(dialogSearch.toLowerCase());
      }
      return true;
    });
  };

  const getFilteredApks = () => {
    if (!dialogSearch) return availableApks;
    return availableApks.filter(apk =>
      apk.filename.toLowerCase().includes(dialogSearch.toLowerCase())
    );
  };

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
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

            <select
              value={filterStatus}
              onChange={(e) => setFilterStatus(e.target.value as any)}
              className="px-3 py-1.5 rounded-md bg-discord-dark-3 border border-discord-dark text-sm text-gray-300"
            >
              <option value="all">All</option>
              <option value="connected">Connected</option>
              <option value="disconnected">Disconnected</option>
            </select>

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
          {filteredDevices.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full">
              <Wifi className="h-16 w-16 text-gray-600 mb-4" />
              <h3 className="text-lg font-semibold text-white mb-2">No devices found</h3>
              <p className="text-gray-400 text-sm text-center max-w-md">
                Make sure your Quest devices have SnorlaxClient running
              </p>
            </div>
          ) : (
            <div
              className={cn(
                viewMode === 'grid'
                  ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'
                  : 'space-y-3'
              )}
            >
              {filteredDevices.map((device) => (
                <DeviceCard
                  key={device.info.id}
                  device={device}
                  isSelected={selectedDeviceIds.has(device.info.id)}
                  onToggle={() => toggleDevice(device.info.id)}
                />
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Right: Command Panel */}
      {hasSelection && (
        <div className="w-80 border-l border-discord-dark-2 bg-discord-dark-3 flex flex-col">
          <div className="p-4 border-b border-discord-dark-2">
            <h3 className="font-semibold text-white">Commands</h3>
            <p className="text-xs text-gray-400 mt-1">
              {selectedDeviceIds.size} device{selectedDeviceIds.size > 1 ? 's' : ''} selected
            </p>
          </div>

          {/* Tab Switcher */}
          <div className="flex border-b border-discord-dark-2">
            <button
              className={cn(
                'flex-1 px-4 py-2 text-sm font-medium transition-colors',
                commandTab === 'standard'
                  ? 'text-white bg-discord-dark-2 border-b-2 border-discord-blurple'
                  : 'text-gray-400 hover:text-white'
              )}
              onClick={() => setCommandTab('standard')}
            >
              Standard
            </button>
            <button
              className={cn(
                'flex-1 px-4 py-2 text-sm font-medium transition-colors',
                commandTab === 'dev'
                  ? 'text-white bg-discord-dark-2 border-b-2 border-discord-blurple'
                  : 'text-gray-400 hover:text-white'
              )}
              onClick={() => setCommandTab('dev')}
            >
              Developer
            </button>
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {commandTab === 'standard' ? (
              <>
                {/* Device Info */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 uppercase font-semibold">Device Info</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => handleCommand(() => DeviceService.requestBattery(selectedIds), 'Get Battery')}
                      disabled={loading}
                    >
                      <Battery className="h-4 w-4 mr-2" />
                      Get Battery
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => handleCommand(() => DeviceService.getVolume(selectedIds), 'Get Volume')}
                      disabled={loading}
                    >
                      <Volume2 className="h-4 w-4 mr-2" />
                      Get Volume
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => handleCommand(() => DeviceService.pingDevices(selectedIds), 'Ping', false)}
                      disabled={loading}
                    >
                      <Wifi className="h-4 w-4 mr-2" />
                      Ping
                    </Button>
                  </div>
                </div>

                {/* App Management */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 uppercase font-semibold">App Management</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openAppListDialog('launch')}
                      disabled={loading}
                    >
                      <PlayCircle className="h-4 w-4 mr-2" />
                      Launch App
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openAppListDialog('uninstall')}
                      disabled={loading}
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      Uninstall App
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openSimpleInputDialog('remote-apk')}
                      disabled={loading}
                    >
                      <Download className="h-4 w-4 mr-2" />
                      Install from URL
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={openApkPickerDialog}
                      disabled={loading}
                    >
                      <Upload className="h-4 w-4 mr-2" />
                      Install Local APK
                    </Button>
                  </div>
                </div>

                {/* Device Control */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 uppercase font-semibold">Device Control</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openSimpleInputDialog('volume')}
                      disabled={loading}
                    >
                      <Volume2 className="h-4 w-4 mr-2" />
                      Set Volume
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => {
                        if (confirm(`Restart ${selectedDeviceIds.size} device(s)?`)) {
                          handleCommand(() => DeviceService.restartDevices(selectedIds), 'Restart');
                        }
                      }}
                      disabled={loading}
                    >
                      <Power className="h-4 w-4 mr-2" />
                      Restart Device
                    </Button>
                  </div>
                </div>
              </>
            ) : (
              <>
                {/* Developer Commands */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 uppercase font-semibold">Developer Tools</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => handleCommand(() => DeviceService.getInstalledApps(selectedIds), 'Get Apps')}
                      disabled={loading}
                    >
                      <List className="h-4 w-4 mr-2" />
                      Get Installed Apps
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openSimpleInputDialog('shell')}
                      disabled={loading}
                    >
                      <Terminal className="h-4 w-4 mr-2" />
                      Shell Command
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openSimpleInputDialog('launch-manual')}
                      disabled={loading}
                    >
                      <FileText className="h-4 w-4 mr-2" />
                      Launch by Package
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => openSimpleInputDialog('uninstall-manual')}
                      disabled={loading}
                    >
                      <FileText className="h-4 w-4 mr-2" />
                      Uninstall by Package
                    </Button>
                  </div>
                </div>
              </>
            )}
          </div>
        </div>
      )}

      {/* Simple Input Dialog */}
      {showSimpleInputDialog && (
        <DialogOverlay onClose={() => setShowSimpleInputDialog(false)}>
          <Card className="bg-discord-dark-2 border-discord-dark w-96">
            <CardHeader>
              <h3 className="text-lg font-semibold text-white">
                {dialogType === 'launch-manual' && 'Launch App by Package'}
                {dialogType === 'uninstall-manual' && 'Uninstall App by Package'}
                {dialogType === 'shell' && 'Execute Shell Command'}
                {dialogType === 'volume' && 'Set Volume'}
                {dialogType === 'remote-apk' && 'Install APK from URL'}
              </h3>
              <p className="text-sm text-gray-400">
                For {selectedDeviceIds.size} device(s)
              </p>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="text-sm text-gray-300 mb-2 block">
                  {(dialogType === 'launch-manual' || dialogType === 'uninstall-manual') && 'Package Name'}
                  {dialogType === 'shell' && 'Shell Command'}
                  {dialogType === 'volume' && 'Volume Level (0-100)'}
                  {dialogType === 'remote-apk' && 'APK URL'}
                </label>
                <Input
                  value={dialogInput}
                  onChange={(e) => setDialogInput(e.target.value)}
                  placeholder={
                    (dialogType === 'launch-manual' || dialogType === 'uninstall-manual') ? 'com.example.app' :
                    dialogType === 'shell' ? 'ls -la' :
                    dialogType === 'volume' ? '50' :
                    'https://example.com/app.apk'
                  }
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') executeSimpleCommand();
                    if (e.key === 'Escape') setShowSimpleInputDialog(false);
                  }}
                  autoFocus
                />
              </div>
              <div className="flex gap-2">
                <Button
                  onClick={executeSimpleCommand}
                  disabled={loading || !dialogInput.trim()}
                  className="flex-1"
                >
                  Execute
                </Button>
                <Button
                  variant="outline"
                  onClick={() => setShowSimpleInputDialog(false)}
                  disabled={loading}
                >
                  Cancel
                </Button>
              </div>
            </CardContent>
          </Card>
        </DialogOverlay>
      )}

      {/* App List Dialog */}
      {showAppListDialog && (
        <DialogOverlay onClose={() => {
          setShowAppListDialog(false);
          setSelectedApp(null);
        }}>
          <Card className="bg-discord-dark-2 border-discord-dark w-[500px] max-h-[600px] flex flex-col">
            <CardHeader>
              <h3 className="text-lg font-semibold text-white">
                {dialogType === 'launch' ? 'Select App to Launch' : 'Select App to Uninstall'}
              </h3>
              <p className="text-sm text-gray-400">
                For {selectedDeviceIds.size} device(s)
              </p>
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
                {getFilteredApps().map((app) => (
                  <button
                    key={app}
                    className={cn(
                      "w-full px-6 py-3 text-left hover:bg-discord-dark-3 transition-colors",
                      selectedApp === app && "bg-discord-blurple/20 border-l-2 border-discord-blurple"
                    )}
                    onClick={() => setSelectedApp(app)}
                    disabled={loading}
                  >
                    <p className="text-sm text-white font-medium">{extractAppName(app)}</p>
                    <p className="text-xs text-gray-400 mt-0.5 font-mono">{app}</p>
                  </button>
                ))}
                {getFilteredApps().length === 0 && (
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
            <div className="p-4 border-t border-discord-dark flex gap-2">
              <Button
                onClick={() => {
                  if (selectedApp) {
                    executeAppCommand(selectedApp);
                  }
                }}
                disabled={loading || !selectedApp}
                className="flex-1"
              >
                {dialogType === 'launch' ? 'Launch' : 'Uninstall'}
              </Button>
              <Button
                variant="outline"
                onClick={() => {
                  setShowAppListDialog(false);
                  setSelectedApp(null);
                }}
                disabled={loading}
              >
                Cancel
              </Button>
            </div>
          </Card>
        </DialogOverlay>
      )}

      {/* APK Picker Dialog */}
      {showApkListDialog && (
        <DialogOverlay onClose={() => setShowApkListDialog(false)}>
          <Card className="bg-discord-dark-2 border-discord-dark w-[500px] max-h-[600px] flex flex-col">
            <CardHeader>
              <h3 className="text-lg font-semibold text-white">Select APK to Install</h3>
              <p className="text-sm text-gray-400">
                For {selectedDeviceIds.size} device(s)
              </p>
              <div className="relative mt-2">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
                <Input
                  value={dialogSearch}
                  onChange={(e) => setDialogSearch(e.target.value)}
                  placeholder="Search APKs..."
                  className="pl-10"
                />
              </div>
            </CardHeader>
            <CardContent className="flex-1 overflow-y-auto p-0">
              <div className="divide-y divide-discord-dark">
                {getFilteredApks().map((apk) => (
                  <button
                    key={apk.filename}
                    className={cn(
                      "w-full px-6 py-3 text-left hover:bg-discord-dark-3 transition-colors",
                      selectedApk?.filename === apk.filename && "bg-discord-blurple/20 border-l-2 border-discord-blurple"
                    )}
                    onClick={() => setSelectedApk(apk)}
                    disabled={loading}
                  >
                    <div className="flex items-center justify-between">
                      <p className="text-sm text-white font-medium">{apk.filename}</p>
                      <Badge variant="secondary" className="text-xs">
                        {formatFileSize(apk.size_bytes)}
                      </Badge>
                    </div>
                  </button>
                ))}
                {getFilteredApks().length === 0 && (
                  <div className="px-6 py-8 text-center text-gray-400">
                    No APK files found
                  </div>
                )}
              </div>
            </CardContent>
            <div className="p-4 border-t border-discord-dark flex gap-2">
              <Button
                onClick={async () => {
                  if (selectedApk) {
                    await executeApkInstall(selectedApk);
                  }
                }}
                disabled={loading || !selectedApk}
                className="flex-1"
              >
                Install
              </Button>
              <Button
                variant="outline"
                onClick={() => {
                  setShowApkListDialog(false);
                  setSelectedApk(null);
                }}
                disabled={loading}
              >
                Cancel
              </Button>
            </div>
          </Card>
        </DialogOverlay>
      )}
    </div>
  );
}

function DialogOverlay({ children, onClose }: { children: React.ReactNode; onClose: () => void }) {
  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50" onClick={onClose}>
      <div onClick={(e) => e.stopPropagation()}>
        {children}
      </div>
    </div>
  );
}

interface DeviceCardProps {
  device: DeviceState;
  isSelected: boolean;
  onToggle: () => void;
}

function DeviceCard({ device, isSelected, onToggle }: DeviceCardProps) {
  const [isEditingName, setIsEditingName] = useState(false);
  const [editedName, setEditedName] = useState(device.info.custom_name || '');
  const [isSavingName, setIsSavingName] = useState(false);
  const setDevices = useDeviceStore((state) => state.setDevices);

  useEffect(() => {
    setEditedName(device.info.custom_name || '');
  }, [device.info.custom_name]);

  const handleSaveName = async () => {
    if (isSavingName) return;

    setIsSavingName(true);
    try {
      const nameToSave = editedName.trim() || null;
      await DeviceService.setDeviceName(device.info.serial, nameToSave);
      setIsEditingName(false);

      const devices = await DeviceService.getDevices();
      setDevices(devices);
    } catch (error) {
      toast.error(`Failed to rename device: ${error}`);
    } finally {
      setIsSavingName(false);
    }
  };

  const handleCancelEdit = () => {
    setEditedName(device.info.custom_name || '');
    setIsEditingName(false);
  };

  return (
    <Card
      className={cn(
        'group bg-discord-dark-2 border-discord-dark cursor-pointer transition-all hover:border-discord-blurple',
        isSelected && 'border-discord-blurple ring-1 ring-discord-blurple'
      )}
      onClick={onToggle}
    >
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-3 flex-1">
            <input
              type="checkbox"
              checked={isSelected}
              onChange={onToggle}
              onClick={(e) => e.stopPropagation()}
              className="h-4 w-4 rounded border-discord-dark bg-discord-dark-3"
            />
            <div className="flex-1">
              <div className="flex items-center gap-2">
                {isEditingName ? (
                  <div className="flex items-center gap-2 flex-1" onClick={(e) => e.stopPropagation()}>
                    <Input
                      value={editedName}
                      onChange={(e) => setEditedName(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter') handleSaveName();
                        if (e.key === 'Escape') handleCancelEdit();
                      }}
                      placeholder={device.info.model}
                      className="h-7 text-sm"
                      autoFocus
                      disabled={isSavingName}
                    />
                    <button
                      onClick={handleSaveName}
                      disabled={isSavingName}
                      className="text-green-400 hover:text-green-300 transition-colors"
                    >
                      <Check className="h-4 w-4" />
                    </button>
                    <button
                      onClick={handleCancelEdit}
                      disabled={isSavingName}
                      className="text-gray-400 hover:text-gray-300 transition-colors"
                    >
                      <X className="h-4 w-4" />
                    </button>
                  </div>
                ) : (
                  <>
                    <div className="flex items-center gap-2">
                      <h3 className="font-semibold text-white">
                        {device.info.custom_name || device.info.model}
                      </h3>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setIsEditingName(true);
                        }}
                        className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-white transition-all"
                      >
                        <Pencil className="h-3 w-3" />
                      </button>
                    </div>
                    <Badge variant={device.is_connected ? 'success' : 'secondary'} className="text-xs">
                      {device.is_connected ? 'Online' : 'Offline'}
                    </Badge>
                  </>
                )}
              </div>
              {!isEditingName && (
                <p className="text-xs text-gray-400 mt-1">{device.info.serial}</p>
              )}
            </div>
          </div>
          <div className={`h-2 w-2 rounded-full ${getStatusColor(device.is_connected)} ${device.is_connected ? 'pulse-dot' : ''}`} />
        </div>
      </CardHeader>
      <CardContent className="space-y-2">
        <div className="flex items-center justify-between text-xs">
          <span className="text-gray-400">IP</span>
          <span className="text-gray-300 font-mono">{device.info.ip}</span>
        </div>
        {device.battery && (
          <div className="flex items-center justify-between text-xs">
            <span className="text-gray-400">Battery</span>
            <div className="flex items-center gap-1">
              <span className={cn('font-medium', getBatteryColor(device.battery.headset_level))}>
                {device.battery.headset_level}%
              </span>
              {device.battery.is_charging && <span className="text-yellow-500 text-sm">⚡</span>}
            </div>
          </div>
        )}
        {device.volume && (
          <div className="flex items-center justify-between text-xs">
            <span className="text-gray-400">Volume</span>
            <span className="text-gray-300">{device.volume.volume_percentage}%</span>
          </div>
        )}
        <div className="flex items-center justify-between text-xs">
          <span className="text-gray-400">Last Seen</span>
          <span className="text-gray-300">{formatDate(device.info.last_seen)}</span>
        </div>
        {device.command_history.length > 0 && (
          <div className="pt-2 border-t border-discord-dark">
            <div className="text-xs text-gray-400 mb-1">Last Command</div>
            <div className={cn(
              'text-xs p-1.5 rounded bg-discord-dark-3',
              device.command_history[0].success ? 'text-green-400' : 'text-red-400'
            )}>
              <span className="font-medium">{device.command_history[0].command_type}</span>
              <br />
              <span className="text-gray-400">{device.command_history[0].message}</span>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
