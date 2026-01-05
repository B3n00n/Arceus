import { Wifi } from 'lucide-react';
import { DeviceCard } from './DeviceCard';
import type { DeviceState } from '@/types/device.types';

interface DeviceListProps {
  devices: DeviceState[];
  selectedDeviceIds: Set<string>;
  onToggleDevice: (deviceId: string) => void;
}

export function DeviceList({ devices, selectedDeviceIds, onToggleDevice }: DeviceListProps) {
  if (devices.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-full">
        <Wifi className="h-16 w-16 text-grey-400 mb-4" />
        <h3 className="text-lg font-semibold text-grey-200 mb-2">No devices found</h3>
        <p className="text-grey-300 text-sm text-center max-w-md">
          Make sure your Quest devices have SnorlaxClient running
        </p>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-3">
      {devices.map((device) => (
        <DeviceCard
          key={device.info.id}
          device={device}
          isSelected={selectedDeviceIds.has(device.info.id)}
          onToggle={() => onToggleDevice(device.info.id)}
        />
      ))}
    </div>
  );
}
