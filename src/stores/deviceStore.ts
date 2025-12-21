import { create } from 'zustand';
import type { DeviceState } from '@/types/device.types';
import { eventService } from '@/services/eventService';

interface DeviceStoreState {
  devices: DeviceState[];
  selectedDeviceIds: Set<string>;
  searchQuery: string;

  setDevices: (devices: DeviceState[]) => void;
  updateDevice: (device: DeviceState) => void;
  addOrUpdateDevice: (device: DeviceState) => void;
  removeDevice: (deviceId: string) => void;

  setSelectedDeviceIds: (ids: Set<string>) => void;
  toggleDevice: (deviceId: string) => void;
  selectAll: () => void;
  clearSelection: () => void;

  setSearchQuery: (query: string) => void;

  getFilteredDevices: () => DeviceState[];
  getSelectedDevices: () => DeviceState[];
}

export const useDeviceStore = create<DeviceStoreState>((set, get) => ({
  devices: [],
  selectedDeviceIds: new Set(),
  searchQuery: '',

  setDevices: (devices) => set({ devices }),

  updateDevice: (device) => set((state) => ({
    devices: state.devices.map((d) =>
      d.info.id === device.info.id ? device : d
    ),
  })),

  addOrUpdateDevice: (device) => set((state) => {
    const existingIndex = state.devices.findIndex((d) => d.info.id === device.info.id);
    if (existingIndex >= 0) {
      const newDevices = [...state.devices];
      newDevices[existingIndex] = device;
      return { devices: newDevices };
    } else {
      return { devices: [...state.devices, device] };
    }
  }),

  removeDevice: (deviceId) => set((state) => ({
    devices: state.devices.filter((d) => d.info.id !== deviceId),
    selectedDeviceIds: new Set(
      Array.from(state.selectedDeviceIds).filter((id) => id !== deviceId)
    ),
  })),

  setSelectedDeviceIds: (ids) => set({ selectedDeviceIds: ids }),

  toggleDevice: (deviceId) => set((state) => {
    const newSelection = new Set(state.selectedDeviceIds);
    if (newSelection.has(deviceId)) {
      newSelection.delete(deviceId);
    } else {
      newSelection.add(deviceId);
    }
    return { selectedDeviceIds: newSelection };
  }),

  selectAll: () => set((state) => ({
    selectedDeviceIds: new Set(state.devices.map((d) => d.info.id)),
  })),

  clearSelection: () => set({ selectedDeviceIds: new Set() }),

  setSearchQuery: (query) => set({ searchQuery: query }),

  getFilteredDevices: () => {
    const { devices, searchQuery } = get();

    return devices.filter((device) => {
      const matchesSearch =
        searchQuery === '' ||
        device.info.model.toLowerCase().includes(searchQuery.toLowerCase()) ||
        device.info.serial.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (device.info.customName?.toLowerCase().includes(searchQuery.toLowerCase()) ?? false);

      return matchesSearch;
    });
  },

  getSelectedDevices: () => {
    const { devices, selectedDeviceIds } = get();
    return devices.filter((d) => selectedDeviceIds.has(d.info.id));
  },
}));

// Helper to update device fields
const updateDeviceField = (deviceId: string, updater: (device: DeviceState) => DeviceState) => {
  const store = useDeviceStore.getState();
  const device = store.devices.find((d) => d.info.id === deviceId);
  if (device) {
    store.updateDevice(updater(device));
  }
};

eventService.subscribe((event) => {
  const store = useDeviceStore.getState();

  switch (event.type) {
    case 'deviceConnected':
      store.addOrUpdateDevice(event.device);
      break;

    case 'deviceDisconnected':
      store.removeDevice(event.deviceId);
      break;

    case 'deviceUpdated':
      store.updateDevice(event.device);
      break;

    case 'deviceNameChanged':
      updateDeviceField(event.deviceId, (device) => ({
        ...device,
        info: { ...device.info, customName: event.newName },
      }));
      break;

    case 'batteryUpdated':
      updateDeviceField(event.deviceId, (device) => ({
        ...device,
        battery: event.batteryInfo,
      }));
      break;

    case 'volumeUpdated':
      updateDeviceField(event.deviceId, (device) => ({
        ...device,
        volume: event.volumeInfo,
      }));
      break;

    case 'operationProgress':
      updateDeviceField(event.deviceId, (device) => ({
        ...device,
        operationProgress: event.progress,
      }));

      // Auto-hide install completion/failure after delay
      if (
        event.progress.operationType === 'install' &&
        (event.progress.stage === 'completed' || event.progress.stage === 'failed')
      ) {
        setTimeout(() => {
          const device = store.devices.find((d) => d.info.id === event.deviceId);
          if (device?.operationProgress?.operationId === event.progress.operationId) {
            updateDeviceField(event.deviceId, (d) => ({ ...d, operationProgress: null }));
          }
        }, 2000);
      }
      break;
  }
});
