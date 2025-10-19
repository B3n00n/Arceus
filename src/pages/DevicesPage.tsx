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
} from 'lucide-react';
import { useDeviceStore } from '@/stores/deviceStore';
import { DeviceService } from '@/services/deviceService';
import { useTauriEvent } from '@/hooks/useTauriEvent';
import { formatDate, getBatteryColor, getStatusColor } from '@/lib/formatting';
import { cn } from '@/lib/cn';
import { toast } from 'sonner';
import type { DeviceState } from '@/types/device.types';

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
  const updateDevice = useDeviceStore((state) => state.updateDevice);

  const [loading, setLoading] = useState(false);
  const [showCommandDialog, setShowCommandDialog] = useState(false);
  const [commandType, setCommandType] = useState<string>('');
  const [commandInput, setCommandInput] = useState('');

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

  useTauriEvent<DeviceState>('device-connected', (device) => {
    updateDevice(device);
    loadDevices();
    toast.success(`${device.info.model} connected`);
  });

  useTauriEvent('device-disconnected', () => {
    loadDevices();
  });

  useTauriEvent('battery-updated', () => {
    loadDevices();
  });

  useTauriEvent('volume-updated', () => {
    loadDevices();
  });

  const filteredDevices = getFilteredDevices();
  const hasSelection = selectedDeviceIds.size > 0;
  const selectedIds = Array.from(selectedDeviceIds);

  const handleCommand = async (action: () => Promise<void>, actionName: string) => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }

    setLoading(true);
    try {
      await action();
      toast.success(`${actionName} sent to ${selectedDeviceIds.size} device(s)`);
    } catch (error) {
      toast.error(`${actionName} failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const openCommandDialog = (type: string) => {
    if (selectedDeviceIds.size === 0) {
      toast.error('Please select at least one device');
      return;
    }
    setCommandType(type);
    setCommandInput('');
    setShowCommandDialog(true);
  };

  const executeCommandDialog = async () => {
    if (!commandInput.trim()) {
      toast.error('Please enter a value');
      return;
    }

    setLoading(true);
    try {
      switch (commandType) {
        case 'launch':
          await DeviceService.launchApp(selectedIds, commandInput);
          toast.success(`Launching ${commandInput}`);
          break;
        case 'uninstall':
          await DeviceService.uninstallApp(selectedIds, commandInput);
          toast.success(`Uninstalling ${commandInput}`);
          break;
        case 'shell':
          await DeviceService.executeShell(selectedIds, commandInput);
          toast.success('Shell command executed');
          break;
        case 'volume':
          const vol = parseInt(commandInput);
          if (isNaN(vol) || vol < 0 || vol > 100) {
            toast.error('Volume must be 0-100');
            return;
          }
          await DeviceService.setVolume(selectedIds, vol);
          toast.success(`Volume set to ${vol}%`);
          break;
        case 'remote-apk':
          await DeviceService.installRemoteApk(selectedIds, commandInput);
          toast.success('Installing APK from URL');
          break;
        case 'local-apk':
          await DeviceService.installLocalApk(selectedIds, commandInput);
          toast.success('Installing local APK');
          break;
      }
      setShowCommandDialog(false);
      setCommandInput('');
    } catch (error) {
      toast.error(`Command failed: ${error}`);
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
            <Button onClick={loadDevices} variant="outline" size="sm" disabled={loading}>
              <RefreshCw className={cn('h-4 w-4 mr-2', loading && 'animate-spin')} />
              Refresh
            </Button>
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

          <div className="flex-1 overflow-y-auto p-4 space-y-4">
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
                  onClick={() => handleCommand(() => DeviceService.pingDevices(selectedIds), 'Ping')}
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
                  onClick={() => openCommandDialog('launch')}
                  disabled={loading}
                >
                  <PlayCircle className="h-4 w-4 mr-2" />
                  Launch App
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full justify-start"
                  onClick={() => openCommandDialog('uninstall')}
                  disabled={loading}
                >
                  <Trash2 className="h-4 w-4 mr-2" />
                  Uninstall App
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full justify-start"
                  onClick={() => openCommandDialog('remote-apk')}
                  disabled={loading}
                >
                  <Download className="h-4 w-4 mr-2" />
                  Install from URL
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full justify-start"
                  onClick={() => openCommandDialog('local-apk')}
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
                  onClick={() => openCommandDialog('volume')}
                  disabled={loading}
                >
                  <Volume2 className="h-4 w-4 mr-2" />
                  Set Volume
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full justify-start"
                  onClick={() => openCommandDialog('shell')}
                  disabled={loading}
                >
                  <Terminal className="h-4 w-4 mr-2" />
                  Shell Command
                </Button>
                <Button
                  variant="destructive"
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
          </div>
        </div>
      )}

      {/* Command Input Dialog */}
      {showCommandDialog && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <Card className="bg-discord-dark-2 border-discord-dark w-96">
            <CardHeader>
              <h3 className="text-lg font-semibold text-white">
                {commandType === 'launch' && 'Launch App'}
                {commandType === 'uninstall' && 'Uninstall App'}
                {commandType === 'shell' && 'Execute Shell Command'}
                {commandType === 'volume' && 'Set Volume'}
                {commandType === 'remote-apk' && 'Install APK from URL'}
                {commandType === 'local-apk' && 'Install Local APK'}
              </h3>
              <p className="text-sm text-gray-400">
                For {selectedDeviceIds.size} device(s)
              </p>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="text-sm text-gray-300 mb-2 block">
                  {commandType === 'launch' && 'Package Name'}
                  {commandType === 'uninstall' && 'Package Name'}
                  {commandType === 'shell' && 'Shell Command'}
                  {commandType === 'volume' && 'Volume Level (0-100)'}
                  {commandType === 'remote-apk' && 'APK URL'}
                  {commandType === 'local-apk' && 'APK Filename'}
                </label>
                <Input
                  value={commandInput}
                  onChange={(e) => setCommandInput(e.target.value)}
                  placeholder={
                    commandType === 'launch' ? 'com.example.app' :
                    commandType === 'uninstall' ? 'com.example.app' :
                    commandType === 'shell' ? 'ls -la' :
                    commandType === 'volume' ? '50' :
                    commandType === 'remote-apk' ? 'https://example.com/app.apk' :
                    'app.apk'
                  }
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') executeCommandDialog();
                    if (e.key === 'Escape') setShowCommandDialog(false);
                  }}
                  autoFocus
                />
              </div>
              <div className="flex gap-2">
                <Button
                  onClick={executeCommandDialog}
                  disabled={loading || !commandInput.trim()}
                  className="flex-1"
                >
                  Execute
                </Button>
                <Button
                  variant="outline"
                  onClick={() => setShowCommandDialog(false)}
                  disabled={loading}
                >
                  Cancel
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}

interface DeviceCardProps {
  device: DeviceState;
  isSelected: boolean;
  onToggle: () => void;
}

function DeviceCard({ device, isSelected, onToggle }: DeviceCardProps) {
  return (
    <Card
      className={cn(
        'bg-discord-dark-2 border-discord-dark cursor-pointer transition-all hover:border-discord-blurple',
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
                <h3 className="font-semibold text-white">
                  {device.info.custom_name || device.info.model}
                </h3>
                <Badge variant={device.is_connected ? 'success' : 'secondary'} className="text-xs">
                  {device.is_connected ? 'Online' : 'Offline'}
                </Badge>
              </div>
              <p className="text-xs text-gray-400 mt-1">{device.info.serial}</p>
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
