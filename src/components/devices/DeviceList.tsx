import { Wifi } from 'lucide-react';
import { cn } from '@/lib/cn';
import { DeviceCard } from './DeviceCard';
import type { DeviceState } from '@/types/device.types';

interface DeviceListProps {
  devices: DeviceState[];
  selectedDeviceIds: Set<string>;
  viewMode: 'grid' | 'list';
  onToggleDevice: (deviceId: string) => void;
}

export function DeviceList({ devices, selectedDeviceIds, viewMode, onToggleDevice }: DeviceListProps) {
  if (devices.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-full">
        <Wifi className="h-16 w-16 text-gray-600 mb-4" />
        <h3 className="text-lg font-semibold text-white mb-2">No devices found</h3>
        <p className="text-gray-400 text-sm text-center max-w-md">
          Make sure your Quest devices have SnorlaxClient running
        </p>
      </div>
    );
  }

  return (
    <div
      className={cn(
        viewMode === 'grid'
          ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'
          : 'space-y-3'
      )}
    >
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
