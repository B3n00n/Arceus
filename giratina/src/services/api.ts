import axios from 'axios';
import type { AxiosInstance } from 'axios';
import type {
  Arcade,
  CreateArcadeRequest,
  UpdateArcadeRequest,
  Game,
  CreateGameRequest,
  UpdateGameRequest,
  GameVersion,
  GameVersionWithChannels,
  CreateGameVersionRequest,
  UpdateGameVersionRequest,
  ReleaseChannel,
  CreateChannelRequest,
  UpdateChannelRequest,
  SnorlaxVersion,
  CreateSnorlaxVersionRequest,
  GyrosVersion,
  CreateGyrosVersionRequest,
} from '../types';

class AlakazamAPI {
  private client: AxiosInstance;

  constructor(baseURL: string = import.meta.env.VITE_ALAKAZAM_API_URL || 'http://localhost:8080') {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    // For local development, send a mock IAP header
    if (import.meta.env.DEV) {
      headers['X-Goog-Authenticated-User-Email'] = 'accounts.google.com:dev@combatica.com';
    }

    this.client = axios.create({
      baseURL,
      headers,
      // withCredentials only needed for production IAP
      // withCredentials: true,
    });
  }

  // Arcade endpoints
  async getArcades(): Promise<Arcade[]> {
    const response = await this.client.get('/api/admin/arcades');
    return response.data;
  }

  async getArcade(id: number): Promise<Arcade> {
    const response = await this.client.get(`/api/admin/arcades/${id}`);
    return response.data;
  }

  async createArcade(data: CreateArcadeRequest): Promise<Arcade> {
    const response = await this.client.post('/api/admin/arcades', data);
    return response.data;
  }

  async updateArcade(id: number, data: UpdateArcadeRequest): Promise<Arcade> {
    const response = await this.client.put(`/api/admin/arcades/${id}`, data);
    return response.data;
  }

  async deleteArcade(id: number): Promise<void> {
    await this.client.delete(`/api/admin/arcades/${id}`);
  }

  // Game endpoints
  async getGames(): Promise<Game[]> {
    const response = await this.client.get('/api/admin/games');
    return response.data;
  }

  async getGame(id: number): Promise<Game> {
    const response = await this.client.get(`/api/admin/games/${id}`);
    return response.data;
  }

  async createGame(data: CreateGameRequest): Promise<Game> {
    const response = await this.client.post('/api/admin/games', data);
    return response.data;
  }

  async updateGame(id: number, data: UpdateGameRequest): Promise<Game> {
    const response = await this.client.put(`/api/admin/games/${id}`, data);
    return response.data;
  }

  async deleteGame(id: number): Promise<void> {
    await this.client.delete(`/api/admin/games/${id}`);
  }

  async uploadGameBackground(
    gameId: number,
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<{ message: string }> {
    const urlResponse = await this.client.post(`/api/admin/games/${gameId}/background/generate-upload-url`);
    const { upload_url } = urlResponse.data;

    await axios.put(upload_url, file, {
      headers: {
        'Content-Type': 'image/jpeg',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(percentCompleted);
        }
      },
    });

    return { message: 'Background uploaded successfully' };
  }

  // Game Version endpoints
  async getGameVersions(gameId: number): Promise<GameVersionWithChannels[]> {
    const response = await this.client.get(`/api/admin/games/${gameId}/versions`);
    return response.data;
  }

  async getGameVersion(gameId: number, versionId: number): Promise<GameVersionWithChannels> {
    const response = await this.client.get(`/api/admin/games/${gameId}/versions/${versionId}`);
    return response.data;
  }

  async createGameVersion(gameId: number, data: CreateGameVersionRequest): Promise<GameVersion> {
    const response = await this.client.post(`/api/admin/games/${gameId}/versions`, data);
    return response.data;
  }

  async updateGameVersion(gameId: number, versionId: number, data: UpdateGameVersionRequest): Promise<GameVersion> {
    const response = await this.client.put(`/api/admin/games/${gameId}/versions/${versionId}`, data);
    return response.data;
  }

  async deleteGameVersion(gameId: number, versionId: number): Promise<void> {
    await this.client.delete(`/api/admin/games/${gameId}/versions/${versionId}`);
  }

  async uploadGameVersion(
    gameId: number,
    version: string,
    files: { file: File; relativePath: string }[],
    onProgress?: (progress: number, filesUploaded: number, totalFiles: number) => void
  ): Promise<GameVersion> {
    const filePaths = files.map(f => f.relativePath);

    // Get batch signed URLs
    const urlResponse = await this.client.post(
      `/api/admin/games/${gameId}/versions/generate-batch-upload-urls`,
      { version, files: filePaths }
    );
    const { gcs_path, files: fileUrls } = urlResponse.data as {
      gcs_path: string;
      files: { path: string; upload_url: string }[];
    };

    const urlMap = new Map(fileUrls.map(f => [f.path, f.upload_url]));

    const totalBytes = files.reduce((sum, f) => sum + f.file.size, 0);
    const fileProgress = new Map<string, number>();
    let completedFiles = 0;

    const updateProgress = () => {
      const uploadedBytes = Array.from(fileProgress.values()).reduce((sum, bytes) => sum + bytes, 0);
      const percent = totalBytes > 0 ? Math.round((uploadedBytes * 100) / totalBytes) : 0;
      onProgress?.(percent, completedFiles, files.length);
    };

    const CONCURRENCY = 6;
    const uploadFile = async (fileInfo: { file: File; relativePath: string }) => {
      const uploadUrl = urlMap.get(fileInfo.relativePath);
      if (!uploadUrl) {
        throw new Error(`No upload URL for ${fileInfo.relativePath}`);
      }

      await axios.put(uploadUrl, fileInfo.file, {
        headers: {
          'Content-Type': this.getContentType(fileInfo.relativePath),
        },
        onUploadProgress: (progressEvent) => {
          fileProgress.set(fileInfo.relativePath, progressEvent.loaded);
          updateProgress();
        },
      });

      fileProgress.set(fileInfo.relativePath, fileInfo.file.size);
      completedFiles++;
      updateProgress();
    };

    for (let i = 0; i < files.length; i += CONCURRENCY) {
      const batch = files.slice(i, i + CONCURRENCY);
      await Promise.all(batch.map(uploadFile));
    }

    const confirmResponse = await this.client.post(
      `/api/admin/games/${gameId}/versions/confirm-upload`,
      { version, gcs_path }
    );

    return confirmResponse.data;
  }

  private getContentType(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() || '';
    const contentTypes: Record<string, string> = {
      'apk': 'application/vnd.android.package-archive',
      'obb': 'application/octet-stream',
      'zip': 'application/zip',
      'json': 'application/json',
      'txt': 'text/plain',
      'xml': 'application/xml',
      'png': 'image/png',
      'jpg': 'image/jpeg',
      'jpeg': 'image/jpeg',
      'gif': 'image/gif',
      'mp4': 'video/mp4',
      'mp3': 'audio/mpeg',
      'wav': 'audio/wav',
      'exe': 'application/x-msdownload',
      'dll': 'application/x-msdownload',
      'bundle': 'application/octet-stream',
      'data': 'application/octet-stream',
    };
    return contentTypes[ext] || 'application/octet-stream';
  }

  async getSnorlaxVersions(): Promise<SnorlaxVersion[]> {
    const response = await this.client.get('/api/admin/snorlax/versions');
    return response.data;
  }

  async createSnorlaxVersion(data: CreateSnorlaxVersionRequest): Promise<SnorlaxVersion> {
    const response = await this.client.post('/api/admin/snorlax/versions', data);
    return response.data;
  }

  async setSnorlaxVersionCurrent(id: number): Promise<void> {
    await this.client.put(`/api/admin/snorlax/versions/${id}/set-current`);
  }

  async deleteSnorlaxVersion(id: number): Promise<void> {
    await this.client.delete(`/api/admin/snorlax/versions/${id}`);
  }

  async uploadSnorlaxApk(
    version: string,
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<SnorlaxVersion> {
    const urlResponse = await this.client.post('/api/admin/snorlax/generate-upload-url', { version });
    const { upload_url, gcs_path } = urlResponse.data;

    await axios.put(upload_url, file, {
      headers: {
        'Content-Type': 'application/vnd.android.package-archive',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(percentCompleted);
        }
      },
    });

    const confirmResponse = await this.client.post('/api/admin/snorlax/confirm-upload', {
      version,
      gcs_path,
    });

    return confirmResponse.data;
  }

  async getGyrosVersions(): Promise<GyrosVersion[]> {
    const response = await this.client.get('/api/admin/gyros/versions');
    return response.data;
  }

  async createGyrosVersion(data: CreateGyrosVersionRequest): Promise<GyrosVersion> {
    const response = await this.client.post('/api/admin/gyros/versions', data);
    return response.data;
  }

  async setGyrosVersionCurrent(id: number): Promise<void> {
    await this.client.put(`/api/admin/gyros/versions/${id}/set-current`);
  }

  async deleteGyrosVersion(id: number): Promise<void> {
    await this.client.delete(`/api/admin/gyros/versions/${id}`);
  }

  async uploadGyrosBinary(
    version: string,
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<GyrosVersion> {
    const urlResponse = await this.client.post('/api/admin/gyros/generate-upload-url', { version });
    const { upload_url, gcs_path } = urlResponse.data;

    await axios.put(upload_url, file, {
      headers: {
        'Content-Type': 'application/octet-stream',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(percentCompleted);
        }
      },
    });

    const confirmResponse = await this.client.post('/api/admin/gyros/confirm-upload', {
      version,
      gcs_path,
    });

    return confirmResponse.data;
  }

  // Release Channel endpoints
  async getChannels(): Promise<ReleaseChannel[]> {
    const response = await this.client.get('/api/admin/channels');
    return response.data;
  }

  async getChannel(id: number): Promise<ReleaseChannel> {
    const response = await this.client.get(`/api/admin/channels/${id}`);
    return response.data;
  }

  async createChannel(data: CreateChannelRequest): Promise<ReleaseChannel> {
    const response = await this.client.post('/api/admin/channels', data);
    return response.data;
  }

  async updateChannel(id: number, data: UpdateChannelRequest): Promise<ReleaseChannel> {
    const response = await this.client.put(`/api/admin/channels/${id}`, data);
    return response.data;
  }

  async deleteChannel(id: number): Promise<void> {
    await this.client.delete(`/api/admin/channels/${id}`);
  }

  async updateArcadeChannel(arcadeId: number, channelId: number): Promise<Arcade> {
    const response = await this.client.put(`/api/admin/arcades/${arcadeId}/channel`, {
      channel_id: channelId,
    });
    return response.data;
  }

  async publishVersionToChannels(
    gameId: number,
    versionId: number,
    channelIds: number[]
  ): Promise<GameVersionWithChannels> {
    const response = await this.client.post(
      `/api/admin/games/${gameId}/versions/${versionId}/publish`,
      { channel_ids: channelIds }
    );
    return response.data;
  }

  async unpublishVersion(gameId: number, versionId: number): Promise<void> {
    await this.client.delete(`/api/admin/games/${gameId}/versions/${versionId}/publish`);
  }
}

export const api = new AlakazamAPI();
