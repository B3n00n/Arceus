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
  CreateGameVersionRequest,
  UpdateGameVersionRequest,
  Assignment,
  CreateAssignmentRequest,
  UpdateAssignmentRequest,
  SnorlaxVersion,
  CreateSnorlaxVersionRequest,
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

  async getArcadeAssignments(arcadeId: number): Promise<Assignment[]> {
    const response = await this.client.get(`/api/admin/arcades/${arcadeId}/assignments`);
    return response.data;
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
    const formData = new FormData();
    formData.append('file', file);

    const response = await this.client.post(`/api/admin/games/${gameId}/background`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(percentCompleted);
        }
      },
    });

    return response.data;
  }

  // Game Version endpoints
  async getGameVersions(gameId: number): Promise<GameVersion[]> {
    const response = await this.client.get(`/api/admin/games/${gameId}/versions`);
    return response.data;
  }

  async getGameVersion(gameId: number, versionId: number): Promise<GameVersion> {
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
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<GameVersion> {
    const formData = new FormData();
    formData.append('version', version);
    formData.append('file', file);

    const response = await this.client.post(
      `/api/admin/games/${gameId}/versions/upload`,
      formData,
      {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
        onUploadProgress: (progressEvent) => {
          if (onProgress && progressEvent.total) {
            const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
            onProgress(percentCompleted);
          }
        },
      }
    );

    return response.data;
  }

  // Assignment endpoints
  async getAssignments(): Promise<Assignment[]> {
    const response = await this.client.get('/api/admin/assignments');
    return response.data;
  }

  async createAssignment(data: CreateAssignmentRequest): Promise<Assignment> {
    const response = await this.client.post('/api/admin/assignments', data);
    return response.data;
  }

  async updateAssignment(id: number, data: UpdateAssignmentRequest): Promise<Assignment> {
    const response = await this.client.put(`/api/admin/assignments/${id}`, data);
    return response.data;
  }

  async deleteAssignment(id: number): Promise<void> {
    await this.client.delete(`/api/admin/assignments/${id}`);
  }

  // Snorlax Version endpoints
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
    const formData = new FormData();
    formData.append('version', version);
    formData.append('file', file);

    const response = await this.client.post('/api/admin/snorlax/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          onProgress(percentCompleted);
        }
      },
    });

    return response.data;
  }
}

export const api = new AlakazamAPI();
