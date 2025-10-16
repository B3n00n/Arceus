export interface DeviceInfo {
  id: string;
  model: string;
  serial: string;
  ip: string;
  custom_name: string | null;
  connected_at: string;
  last_seen: string;
}

export interface BatteryInfo {
  headset_level: number;
  is_charging: boolean;
  last_updated: string;
}

export interface VolumeInfo {
  volume_percentage: number;
  current_volume: number;
  max_volume: number;
  last_updated: string;
}

export interface CommandResult {
  command_type: string;
  success: boolean;
  message: string;
  timestamp: string;
}

export interface DeviceState {
  info: DeviceInfo;
  battery: BatteryInfo | null;
  volume: VolumeInfo | null;
  command_history: CommandResult[];
  is_connected: boolean;
}
