export interface UpdateInfo {
  version: string;
  currentVersion: string;
  body?: string;
  date?: string;
  isAvailable: boolean;
}

export type UpdateStatus =
  | { type: 'Checking' }
  | { type: 'NoUpdate' }
  | { type: 'UpdateAvailable'; data: UpdateInfo }
  | { type: 'Downloading'; data: { progress: number; bytesDownloaded: number; totalBytes: number } }
  | { type: 'Installing' }
  | { type: 'Complete' }
  | { type: 'Error'; data: { message: string } };

export interface UpdateProgress {
  chunkLen: number;
  contentLen?: number;
  downloaded: number;
  percentage?: number;
}