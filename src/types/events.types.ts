export interface CommandResult {
  timestamp: string;
  commandType: string;
  success: boolean;
  message: string;
}

export type ArceusEvent =
  | {
      type: 'installedAppsReceived';
      deviceId: string;
      apps: string[];
    }
  | {
      type: 'commandExecuted';
      deviceId: string;
      result: CommandResult;
    }
  | {
      type: 'batteryUpdated';
      deviceId: string;
      batteryInfo: {
        level: number;
        isCharging: boolean;
      };
    }
  | {
      type: 'volumeUpdated';
      deviceId: string;
      volumeInfo: {
        currentVolume: number;
        maxVolume: number;
      };
    };
