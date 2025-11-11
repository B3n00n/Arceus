import type { DeviceState } from './device.types';

export interface CommandResult {
  timestamp: string;
  commandType: string;
  success: boolean;
  message: string;
}

export type ArceusEvent =
  | {
      type: 'deviceConnected';
      device: DeviceState;
    }
  | {
      type: 'deviceDisconnected';
      deviceId: string;
      serial: string;
    }
  | {
      type: 'deviceUpdated';
      deviceId: string;
    }
  | {
      type: 'batteryUpdated';
      deviceId: string;
      batteryInfo: {
        headsetLevel: number;
        isCharging: boolean;
      };
    }
  | {
      type: 'volumeUpdated';
      deviceId: string;
      volumeInfo: {
        volumePercentage: number;
        currentVolume: number;
        maxVolume: number;
      };
    }
  | {
      type: 'commandExecuted';
      deviceId: string;
      result: CommandResult;
    }
  | {
      type: 'installedAppsReceived';
      deviceId: string;
      apps: string[];
    }
  | {
      type: 'deviceNameChanged';
      deviceId: string;
      serial: string;
      newName: string | null;
    }
  | {
      type: 'serverStarted';
      tcpPort: number;
      httpPort: number;
    }
  | {
      type: 'serverStopped';
    }
  | {
      type: 'httpServerStarted';
      port: number;
      url: string;
    }
  | {
      type: 'error';
      message: string;
      context: string | null;
    }
  | {
      type: 'info';
      message: string;
    }
  | {
      type: 'gameStarted';
      gameName: string;
      processId: number | null;
      contentServerUrl: string;
    }
  | {
      type: 'gameStopped';
      gameName: string;
    };
