import { invoke } from "@tauri-apps/api/core";
import type { Sensor } from "../types/sensor.types";

export class SensorService {
  static async listSensors(): Promise<Sensor[]> {
    return await invoke<Sensor[]>("list_sensors");
  }

  static async getSensorInfo(port: string): Promise<Sensor> {
    return await invoke<Sensor>("get_sensor_info", { port });
  }

  static async uploadFirmware(
    port: string | null,
    firmwarePath: string,
    deviceName: string
  ): Promise<void> {
    await invoke("upload_sensor_firmware", {
      port,
      firmwarePath,
      deviceName,
    });
  }

  static async getMaxNameLength(): Promise<number> {
    return await invoke<number>("get_max_sensor_name_length");
  }

  static async validateFirmware(firmwarePath: string): Promise<boolean> {
    return await invoke<boolean>("validate_sensor_firmware", { firmwarePath });
  }
}
