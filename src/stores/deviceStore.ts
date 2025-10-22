import { create } from 'zustand';
import type { DeviceState } from '@/types/device.types';
import { eventService } from '@/services/eventService';

interface DeviceStoreState {
  devices: DeviceState[];
  selectedDeviceIds: Set<string>;
  searchQuery: string;
  filterStatus: 'all' | 'connected' | 'disconnected';
  viewMode: 'grid' | 'list';

  setDevices: (devices: DeviceState[]) => void;
  updateDevice: (device: DeviceState) => void;
  addOrUpdateDevice: (device: DeviceState) => void;
  removeDevice: (deviceId: string) => void;

  setSelectedDeviceIds: (ids: Set<string>) => void;
  toggleDevice: (deviceId: string) => void;
  selectAll: () => void;
  clearSelection: () => void;

  setSearchQuery: (query: string) => void;
  setFilterStatus: (status: 'all' | 'connected' | 'disconnected') => void;
  setViewMode: (mode: 'grid' | 'list') => void;

  getFilteredDevices: () => DeviceState[];
  getSelectedDevices: () => DeviceState[];
}

export const useDeviceStore = create<DeviceStoreState>((set, get) => ({
  devices: [],
  selectedDeviceIds: new Set(),
  searchQuery: '',
  filterStatus: 'all',
  viewMode: 'grid',

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
  setFilterStatus: (status) => set({ filterStatus: status }),
  setViewMode: (mode) => set({ viewMode: mode }),

  getFilteredDevices: () => {
    const { devices, searchQuery, filterStatus } = get();

    return devices.filter((device) => {
      const matchesSearch =
        searchQuery === '' ||
        device.info.model.toLowerCase().includes(searchQuery.toLowerCase()) ||
        device.info.serial.toLowerCase().includes(searchQuery.toLowerCase()) ||
        device.info.ip.includes(searchQuery) ||
        (device.info.custom_name?.toLowerCase().includes(searchQuery.toLowerCase()) ?? false);

      const matchesStatus =
        filterStatus === 'all' ||
        (filterStatus === 'connected' && device.is_connected) ||
        (filterStatus === 'disconnected' && !device.is_connected);

      return matchesSearch && matchesStatus;
    });
  },

  getSelectedDevices: () => {
    const { devices, selectedDeviceIds } = get();
    return devices.filter((d) => selectedDeviceIds.has(d.info.id));
  },
}));

eventService.subscribe((event) => {
  const store = useDeviceStore.getState();

  switch (event.type) {
    case 'deviceConnected':
      store.addOrUpdateDevice(event.device);
      break;

    case 'deviceDisconnected':
      store.removeDevice(event.deviceId);
      break;

    case 'deviceNameChanged':
      const device = store.devices.find(d => d.info.id === event.deviceId);
      if (device) {
        store.updateDevice({
          ...device,
          info: {
            ...device.info,
            custom_name: event.newName,
          }
        });
      }
      break;
  }
});
