export interface DeviceInfo {
  id: string;
  model: string;
  serial: string;
  version: string;
  customName: string | null;
  connectedAt: string;
  lastSeen: string;
  runningApp: string | null;
  updateAvailable: boolean;
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

export interface DeviceOperationProgress {
  operationType: 'download' | 'install';
  operationId: string;
  stage: 'started' | 'inprogress' | 'completed' | 'failed';
  percentage: number;
}

export interface DeviceState {
  info: DeviceInfo;
  battery: BatteryInfo | null;
  volume: VolumeInfo | null;
  commandHistory: CommandResult[];
  operationProgress: DeviceOperationProgress | null;
}
