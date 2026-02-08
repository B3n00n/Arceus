export type SensorConnectionStatus = 'connected' | 'bootloader' | 'disconnected';

export interface Sensor {
  port: string;
  serial_number?: string;
  mac_address?: string;
  ble_mac_address?: string;
  device_name?: string;
  firmware_version?: string;
  status: SensorConnectionStatus;
}
