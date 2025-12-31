// Alakazam API Type Definitions

export interface Arcade {
  id: number;
  name: string;
  mac_address: string;
  status: string;
  last_seen_at?: string;
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

export interface GameVersionWithGame extends GameVersion {
  game_name: string;
}

export interface Assignment {
  id: number;
  arcade_id: number;
  game_id: number;
  assigned_version_id: number;
  current_version_id?: number;
  updated_at: string;
}

export interface AssignmentWithDetails extends Assignment {
  arcade_name: string;
  game_name: string;
  assigned_version: string;
  current_version?: string;
}

export interface SnorlaxVersion {
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
  mac_address: string;
}

export interface UpdateArcadeRequest {
  name?: string;
  status?: string;
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
  release_date: string;
}

export interface UpdateGameVersionRequest {
  version?: string;
  gcs_path?: string;
  release_date?: string;
}

export interface CreateAssignmentRequest {
  arcade_id: number;
  game_id: number;
  assigned_version_id: number;
}

export interface UpdateAssignmentRequest {
  assigned_version_id: number;
}

export interface CreateSnorlaxVersionRequest {
  version: string;
  gcs_path: string;
  release_date: string;
}

// API Response types
export interface ApiError {
  error: string;
  details?: string;
}
