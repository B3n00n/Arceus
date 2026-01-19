// Alakazam API Type Definitions

export interface ReleaseChannel {
  id: number;
  name: string;
  description: string | null;
  created_at: string;
}

export interface ChannelInfo {
  id: number;
  name: string;
}

export interface Arcade {
  id: number;
  name: string;
  machine_id: string;
  status: string;
  channel_id: number;
  last_seen_at: string | null;
  created_at: string;
}

export interface Game {
  id: number;
  name: string;
  created_at: string;
  background_url?: string;
}

export interface GameVersion {
  id: number;
  game_id: number;
  version: string;
  gcs_path: string;
  release_date: string;
}

export interface GameVersionWithChannels {
  id: number;
  game_id: number;
  version: string;
  gcs_path: string;
  release_date: string;
  channels: ChannelInfo[];
}

export interface GameVersionWithGame extends GameVersionWithChannels {
  game_name: string;
}

export interface SnorlaxVersion {
  id: number;
  version: string;
  gcs_path: string;
  release_date: string;
  is_current: boolean;
  created_at: string;
}

export interface GyrosVersion {
  id: number;
  version: string;
  gcs_path: string;
  release_date: string;
  is_current: boolean;
  created_at: string;
}

// API Request types
export interface CreateArcadeRequest {
  name: string;
  machine_id: string;
  channel_id: number;
}

export interface UpdateArcadeRequest {
  name: string;
  status: string;
}

export interface UpdateArcadeChannelRequest {
  channel_id: number;
}

export interface CreateChannelRequest {
  name: string;
  description?: string;
}

export interface UpdateChannelRequest {
  description?: string;
}

export interface CreateGameRequest {
  name: string;
}

export interface UpdateGameRequest {
  name: string;
}

export interface CreateGameVersionRequest {
  version: string;
  gcs_path: string;
}

export interface UpdateGameVersionRequest {
  version: string;
  gcs_path: string;
}

export interface PublishVersionRequest {
  channel_ids: number[];
}

export interface CreateSnorlaxVersionRequest {
  version: string;
  gcs_path: string;
}

export interface CreateGyrosVersionRequest {
  version: string;
  gcs_path: string;
}

// API Response types
export interface ApiError {
  error: string;
  details?: string;
}
