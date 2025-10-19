import { invoke } from "@tauri-apps/api/core";
import type { ApkInfo } from "@/types/apk.types";

export class ApkService {
  static async listApks(): Promise<ApkInfo[]> {
    return await invoke<ApkInfo[]>("list_apks");
  }

  static async addApk(sourcePath: string): Promise<void> {
    await invoke("add_apk", { sourcePath });
  }

  static async removeApk(filename: string): Promise<void> {
    await invoke("remove_apk", { filename });
  }

  static async openApkFolder(): Promise<void> {
    await invoke("open_apk_folder");
  }
}
