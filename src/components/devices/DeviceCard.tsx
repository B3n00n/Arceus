import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Check, X, Pencil } from 'lucide-react';
import { useDeviceStore } from '@/stores/deviceStore';
import { DeviceService } from '@/services/deviceService';
import { formatDate } from '@/lib/formatting';
import { cn } from '@/lib/cn';
import { toast } from '@/lib/toast';
import type { DeviceState } from '@/types/device.types';
import { DeviceBattery } from '@/components/devices/DeviceBattery';

interface DeviceCardProps {
  device: DeviceState;
  isSelected: boolean;
  onToggle: () => void;
}

export function DeviceCard({ device, isSelected, onToggle }: DeviceCardProps) {
  const [isEditingName, setIsEditingName] = useState(false);
  const [editedName, setEditedName] = useState(device.info.customName || '');
  const [isSavingName, setIsSavingName] = useState(false);
  const setDevices = useDeviceStore((state) => state.setDevices);

  useEffect(() => {
    setEditedName(device.info.customName || '');
  }, [device.info.customName]);

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
    setEditedName(device.info.customName || '');
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
                    <div className="flex items-center gap-2 flex-1">
                      <h3 className="font-semibold text-white">
                        {device.info.customName || device.info.model}
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
                    <Badge variant={device.isConnected ? 'success' : 'destructive'} className="text-xs ml-auto">
                      {device.isConnected ? 'Online' : 'Offline'}
                    </Badge>
                  </>
                )}
              </div>
              {!isEditingName && (
                <p className="text-xs text-gray-400 mt-1">{device.info.serial}</p>
              )}
            </div>
          </div>
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
            <DeviceBattery level={device.battery.headsetLevel} isCharging={device.battery.isCharging} />
          </div>
        )}
        {device.volume && (
          <div className="flex items-center justify-between text-xs">
            <span className="text-gray-400">Volume</span>
            <span className="text-gray-300">{device.volume.volumePercentage}%</span>
          </div>
        )}
        <div className="flex items-center justify-between text-xs">
          <span className="text-gray-400">Last Seen</span>
          <span className="text-gray-300">{formatDate(device.info.lastSeen)}</span>
        </div>
        {device.commandHistory.length > 0 && (
          <div className="pt-2 border-t border-discord-dark">
            <div className="text-xs text-gray-400 mb-1">Last Command</div>
            <div
              className={cn(
                'text-xs p-1.5 rounded bg-discord-dark-3',
                device.commandHistory[0].success ? 'text-green-400' : 'text-red-400'
              )}
            >
              <span className="font-medium">{device.commandHistory[0].commandType}</span>
              <br />
              <span className="text-gray-400">{device.commandHistory[0].message}</span>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
