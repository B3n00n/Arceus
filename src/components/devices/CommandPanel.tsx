import { useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  Battery,
  Wifi,
  Volume2,
  Power,
  Trash2,
  PlayCircle,
  Terminal,
  Download,
  Upload,
  FileText,
  List,
} from 'lucide-react';
import { DeviceService } from '@/services/deviceService';
import { cn } from '@/lib/cn';

type CommandTab = 'standard' | 'dev';

interface CommandPanelProps {
  selectedDeviceIds: Set<string>;
  loading: boolean;
  onOpenAppListDialog: (type: 'launch' | 'uninstall') => void;
  onOpenApkPickerDialog: () => void;
  onOpenSimpleInputDialog: (type: string) => void;
  onHandleCommand: (action: () => Promise<void>, actionName: string, showSuccessNotification?: boolean) => Promise<void>;
}

export function CommandPanel({
  selectedDeviceIds,
  loading,
  onOpenAppListDialog,
  onOpenApkPickerDialog,
  onOpenSimpleInputDialog,
  onHandleCommand,
}: CommandPanelProps) {
  const [commandTab, setCommandTab] = useState<CommandTab>('standard');
  const hasSelection = selectedDeviceIds.size > 0;
  const selectedIds = Array.from(selectedDeviceIds);

  return (
    <div className="w-80 border-l border-discord-dark-2 bg-discord-dark-3 flex flex-col">
      <div className="p-4 border-b border-discord-dark-2">
        <h3 className="font-semibold text-white">Commands</h3>
        <p className="text-xs text-gray-400 mt-1">
          {hasSelection
            ? `${selectedDeviceIds.size} device${selectedDeviceIds.size > 1 ? 's' : ''} selected`
            : 'No device selected'}
        </p>
      </div>

      {!hasSelection ? (
        <div className="flex-1 flex items-center justify-center p-6">
          <div className="text-center">
            <p className="text-gray-400 text-sm">
              Select a device to execute commands
            </p>
          </div>
        </div>
      ) : (
        <>
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
                      onClick={() => onHandleCommand(() => DeviceService.requestBattery(selectedIds), 'Get Battery')}
                      disabled={loading}
                    >
                      <Battery className="h-4 w-4 mr-2" />
                      Get Battery
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onHandleCommand(() => DeviceService.getVolume(selectedIds), 'Get Volume')}
                      disabled={loading}
                    >
                      <Volume2 className="h-4 w-4 mr-2" />
                      Get Volume
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onHandleCommand(() => DeviceService.pingDevices(selectedIds), 'Ping', false)}
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
                      onClick={() => onOpenAppListDialog('launch')}
                      disabled={loading}
                    >
                      <PlayCircle className="h-4 w-4 mr-2" />
                      Launch App
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenAppListDialog('uninstall')}
                      disabled={loading}
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      Uninstall App
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('remote-apk')}
                      disabled={loading}
                    >
                      <Download className="h-4 w-4 mr-2" />
                      Install from URL
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={onOpenApkPickerDialog}
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
                      onClick={() => onOpenSimpleInputDialog('volume')}
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
                          onHandleCommand(() => DeviceService.restartDevices(selectedIds), 'Restart', false);
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
                      onClick={() => onHandleCommand(() => DeviceService.getInstalledApps(selectedIds), 'Get Apps')}
                      disabled={loading}
                    >
                      <List className="h-4 w-4 mr-2" />
                      Get Installed Apps
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('shell')}
                      disabled={loading}
                    >
                      <Terminal className="h-4 w-4 mr-2" />
                      Shell Command
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('launch-manual')}
                      disabled={loading}
                    >
                      <FileText className="h-4 w-4 mr-2" />
                      Launch by Package
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('uninstall-manual')}
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
        </>
      )}
    </div>
  );
}
