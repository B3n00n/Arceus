export interface DeviceInfo {
  id: string;
  model: string;
  serial: string;
  ip: string;
  customName: string | null;
  connectedAt: string;
  lastSeen: string;
}

export interface BatteryInfo {
  headsetLevel: number;
  isCharging: boolean;
}

export interface VolumeInfo {
  volumePercentage: number;
  currentVolume: number;
  maxVolume: number;
}

export interface CommandResult {
  commandType: string;
  success: boolean;
  message: string;
  timestamp: string;
}

export interface DeviceState {
  info: DeviceInfo;
  battery: BatteryInfo | null;
  volume: VolumeInfo | null;
  commandHistory: CommandResult[];
}
