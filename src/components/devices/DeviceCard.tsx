import { useState, useEffect } from 'react';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Checkbox } from '@/components/ui/checkbox';
import { Check, X, Pencil } from 'lucide-react';
import { useDeviceStore } from '@/stores/deviceStore';
import { DeviceService } from '@/services/deviceService';
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
      onClick={onToggle}
      className={cn(
        'group p-4 rounded-lg cursor-pointer transition-all text-gray-300',
        'outline outline-1 outline-offset-[-1px] outline-discord-dark-3 hover:outline-white',
        'min-w-fit flex items-center w-full gap-8',
        isSelected && 'outline-white outline-2 outline-offset-[-2px]'
      )}
    >
{/* Checkbox */}
<div className="flex-shrink-0 flex items-center justify-start">
  <Checkbox
    checked={isSelected}
    onCheckedChange={() => onToggle()}
    className="border-discord-dark-3"
  />
</div>



      {/* Name */}
      <div
        className="group/name flex-[2] min-w-[8rem] flex justify-between items-center gap-2"
        onClick={(e) => {
          e.stopPropagation();
          if (!isEditingName) setIsEditingName(true);
        }}
      >
        <div className="text-white text-sm font-bold flex justify-between items-center gap-2 w-full">
          {isEditingName ? (
            <>
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
                onClick={(e) => {
                  e.stopPropagation();
                  handleSaveName();
                }}
                disabled={isSavingName}
                className="text-green-400 hover:text-green-300 transition-colors"
              >
                <Check className="h-3.5 w-3.5" />
              </button>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleCancelEdit();
                }}
                disabled={isSavingName}
                className="text-gray-400 hover:text-gray-300 transition-colors"
              >
                <X className="h-3.5 w-3.5" />
              </button>
            </>
          ) : (
            <>
              <span className="truncate">{device.info.customName || device.info.model}</span>
              <Pencil className="h-3 w-3 text-gray-400 group-hover/name:text-white transition-all" />
            </>
          )}
        </div>
      </div>

      {/* MAC / Serial */}
      <div className="flex-[1.5] min-w-[8rem] flex justify-start items-center">
        <div className="text-sm truncate">
          {device.info.serial}
        </div>
      </div>

      {/* IP */}
      <div className="flex-[1.5] min-w-[8rem] flex justify-start items-center">
        <div className="text-sm truncate">
          {device.info.ip}
        </div>
      </div>

      {/* Volume */}
      <div className="flex-[0.75] min-w-[5rem] flex justify-start items-center">
        <div className="text-sm text-gray-300">
          {device.volume ? `${device.volume.volumePercentage}%` : '--'}
        </div>
      </div>

      {/* Battery */}
      <div className="flex-[0.75] min-w-[5rem] flex justify-start items-center">
        {device.battery ? (
          <DeviceBattery
            level={device.battery.headsetLevel}
            isCharging={device.battery.isCharging}
          />
        ) : (
          <div className="text-sm font-medium text-gray-300">N/A</div>
        )}
      </div>
    </Card>
  );
}
