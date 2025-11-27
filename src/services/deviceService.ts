import { invoke } from "@tauri-apps/api/core";
import type { DeviceState } from "../types/device.types";

export class DeviceService {
  static async getDevices(): Promise<DeviceState[]> {
    return await invoke<DeviceState[]>("get_devices");
  }

  static async getDevice(deviceId: string): Promise<DeviceState | null> {
    return await invoke<DeviceState | null>("get_device", {
      deviceId
    });
  }

  static async restartDevices(deviceIds: string[]): Promise<void> {
    await invoke("restart_devices", {
      deviceIds
    });
  }

  static async requestBattery(deviceIds: string[]): Promise<void> {
    await invoke("request_battery", {
      deviceIds
    });
  }

  static async getVolume(deviceIds: string[]): Promise<void> {
    await invoke("get_volume", {
      deviceIds
    });
  }

  static async setVolume(deviceIds: string[], level: number): Promise<void> {
    await invoke("set_volume", {
      deviceIds,
      level
    });
  }

  static async pingDevices(deviceIds: string[]): Promise<void> {
    await invoke("ping_devices", {
      deviceIds
    });
  }

  static async launchApp(
    deviceIds: string[],
    packageName: string
  ): Promise<void> {
    await invoke("launch_app", {
      deviceIds,
      packageName
    });
  }

  static async uninstallApp(
    deviceIds: string[],
    packageName: string
  ): Promise<void> {
    await invoke("uninstall_app", {
      deviceIds,
      packageName
    });
  }

  static async executeShell(
    deviceIds: string[],
    command: string
  ): Promise<void> {
    await invoke("execute_shell", {
      deviceIds,
      command
    });
  }

  static async getInstalledApps(deviceIds: string[]): Promise<void> {
    await invoke("get_installed_apps", {
      deviceIds
    });
  }

  static async installRemoteApk(
    deviceIds: string[],
    url: string
  ): Promise<void> {
    await invoke("install_remote_apk", {
      deviceIds,
      url
    });
  }

  static async installLocalApk(
    deviceIds: string[],
    filename: string
  ): Promise<void> {
    await invoke("install_local_apk", {
      deviceIds,
      filename
    });
  }

  static async setDeviceName(
    serial: string,
    name: string | null
  ): Promise<void> {
    await invoke("set_device_name", {
      serial,
      name
    });
  }

  static async closeAllApps(deviceIds: string[]): Promise<void> {
    await invoke("close_all_apps", {
      deviceIds
    });
  }

  static async displayMessage(
    deviceIds: string[],
    message: string
  ): Promise<void> {
    await invoke("display_message", {
      deviceIds,
      message
    });
  }
}
