import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Slider } from '@/components/ui/slider';
import { SegmentedControl } from "@/components/ui/SegmentedControl";
import {
  Battery,
  BellRing,
  Volume2,
  Power,
  Trash2,
  Rocket,
  Terminal,
  Download,
  Upload,
  Package,
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
  onHandleCommand: (
    action: () => Promise<void>,
    actionName: string,
    showSuccessNotification?: boolean
  ) => Promise<void>;
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
  const [isVolumeExpanded, setIsVolumeExpanded] = useState(false);
  const [volumeValue, setVolumeValue] = useState(50);

  const hasSelection = selectedDeviceIds.size > 0;
  const selectedIds = Array.from(selectedDeviceIds);

  const handleSetVolume = async () => {
    await onHandleCommand(
      () => DeviceService.setVolume(selectedIds, volumeValue),
      'Set Volume'
    );
    setIsVolumeExpanded(false);
  };

  return (
    <div className="sm:w-60 2xl:w-80
    min-w-[12rem] max-w-80
 border-l border-discord-dark-2 flex flex-col
  transition-all duration-300">
      {/* Header */}
      <div className="p-4 border-b border-discord-dark-2">
        <h3 className="font-semibold text-white">Commands</h3>
        <p className="text-xs text-gray-400 mt-1">
          {hasSelection
            ? `${selectedDeviceIds.size} device${selectedDeviceIds.size > 1 ? 's' : ''} selected`
            : 'No device selected'}
        </p>
      </div>

      {/* No selection placeholder */}
      {!hasSelection ? (
        <div className="flex-1 flex items-center justify-center p-6">
          <div className="text-center">
            <p className="text-gray-400 text-sm">Select a device to execute commands</p>
          </div>
        </div>
      ) : (
        <>
          {/* Tab Switcher */}
          <SegmentedControl
  options={[
    { label: "Standard", value: "standard" },
    { label: "Developer", value: "dev" },
  ]}
  value={commandTab}
  onChange={(val) => setCommandTab(val as "standard" | "dev")}
/>

          {/* Tab Content */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {commandTab === 'standard' ? (
              <>
                {/* Device Info */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 font-semibold">Device info</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() =>
                        onHandleCommand(
                          () => DeviceService.pingDevices(selectedIds),
                          'Ping',
                          false
                        )
                      }
                      disabled={loading}
                    >
                      <BellRing className="h-4 w-4 mr-2" />
                      Ping device
                    </Button>
                  </div>
                </div>

                {/* App Management */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 font-semibold">App management</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenAppListDialog('launch')}
                      disabled={loading}
                    >
                      <Rocket className="h-4 w-4 mr-2" />
                      Launch app
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenAppListDialog('uninstall')}
                      disabled={loading}
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      Uninstall app
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
                      Install local APK
                    </Button>
                  </div>
                </div>

                {/* Device Control */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 font-semibold">Device control</p>
                  <div className="space-y-2">
                  {/* Set Volume */}
<div className="transition-all duration-300 ease-in-out">
  <button
    onClick={() => setIsVolumeExpanded((prev) => !prev)}
    disabled={loading}
    className={cn(
      'w-full border-2 bg-transparent cursor-pointer',
      'rounded-md px-3 py-2 text-xs',
      'hover:bg-[#7289da]/20 hover:border-[#7289da] hover:text-white',
      isVolumeExpanded
        ? 'text-white border-white hover:bg-transparent'
        : 'text-gray-300 border-gray-600/50'
    )}
  >
    {/* Header row (icon + label + value) */}
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-2">
        <Volume2 className="h-4 w-4 mr-2" />
        <span className="font-medium">Set volume</span>
      </div>
      {isVolumeExpanded && (
        <span className="font-medium text-white">{volumeValue}</span>
      )}
    </div>

    {/* Animated reveal content */}
    <div
      className={cn(
        'overflow-hidden transition-all duration-300 ease-in-out',
        isVolumeExpanded ? 'max-h-40 opacity-100 mt-1 pt-px' : 'max-h-0 opacity-0'
      )}
      onClick={(e) => e.stopPropagation()}
    >
      <Slider
        min={0}
        max={100}
        value={volumeValue}
        onValueChange={setVolumeValue}
        className="w-full mb-3"
      />

      <div className="flex flex-row-reverse gap-2">
        <Button
          size="sm"
          variant="default"
          onClick={(e) => {
            e.stopPropagation();
            handleSetVolume();
          }}
          disabled={loading}          
        >
          Set
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={(e) => {
            e.stopPropagation();
            setIsVolumeExpanded(false);
          }}
          disabled={loading}
          className=""
        >
          Dismiss
        </Button>
      </div>
    </div>
  </button>
</div>

                    {/* Restart device */}
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => {
                        if (confirm(`Restart ${selectedDeviceIds.size} device(s)?`)) {
                          onHandleCommand(
                            () => DeviceService.restartDevices(selectedIds),
                            'Restart',
                            false
                          );
                        }
                      }}
                      disabled={loading}
                    >
                      <Power className="h-4 w-4 mr-2" />
                      Restart device
                    </Button>
                  </div>
                </div>
              </>
            ) : (
              <>
                {/* Developer Commands */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 font-semibold">Developer tools</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() =>
                        onHandleCommand(
                          () => DeviceService.requestBattery(selectedIds),
                          'Get Battery'
                        )
                      }
                      disabled={loading}
                    >
                      <Battery className="h-4 w-4 mr-2" />
                      Get battery
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() =>
                        onHandleCommand(
                          () => DeviceService.getVolume(selectedIds),
                          'Get Volume'
                        )
                      }
                      disabled={loading}
                    >
                      <Volume2 className="h-4 w-4 mr-2" />
                      Get volume
                    </Button>
                  </div>
                </div>

                {/* B3n00n Tools */}
                <div>
                  <p className="text-xs text-gray-400 mb-2 font-semibold">B3n00n tools</p>
                  <div className="space-y-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() =>
                        onHandleCommand(
                          () => DeviceService.getInstalledApps(selectedIds),
                          'Get Apps'
                        )
                      }
                      disabled={loading}
                    >
                      <List className="h-4 w-4 mr-2" />
                      Get installed apps
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('shell')}
                      disabled={loading}
                    >
                      <Terminal className="h-4 w-4 mr-2" />
                      Shell command
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('launch-manual')}
                      disabled={loading}
                    >
                      <Package className="h-4 w-4 mr-2" />
                      Launch by package
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="w-full justify-start"
                      onClick={() => onOpenSimpleInputDialog('uninstall-manual')}
                      disabled={loading}
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      Uninstall by package
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
