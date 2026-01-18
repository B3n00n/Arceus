-- Initial schema for Alakazam VR Arcade Management System
-- Creates all core tables for arcade, game, and version management

-- ============================================================================
-- ARCADES TABLE
-- Represents physical VR arcade installations
-- ============================================================================
CREATE TABLE arcades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    mac_address VARCHAR(17) UNIQUE NOT NULL,  -- MAC address format: XX:XX:XX:XX:XX:XX
    status VARCHAR(50) NOT NULL DEFAULT 'active',  -- active, inactive, maintenance
    last_seen_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for frequent MAC address lookups
CREATE INDEX idx_arcades_mac_address ON arcades(mac_address);
CREATE INDEX idx_arcades_status ON arcades(status);

-- ============================================================================
-- GAMES TABLE
-- Represents VR games available in the system
-- ============================================================================
CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for game name lookups
CREATE INDEX idx_games_name ON games(name);

-- ============================================================================
-- GAME_VERSIONS TABLE
-- Specific versions of games stored in GCS
-- ============================================================================
CREATE TABLE game_versions (
    id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    version VARCHAR(50) NOT NULL,
    gcs_path VARCHAR(512) NOT NULL,  -- Path in GCS bucket
    release_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(game_id, version)  -- Each game can have only one version with a given version string
);

-- Indexes for frequent queries
CREATE INDEX idx_game_versions_game_id ON game_versions(game_id);
CREATE INDEX idx_game_versions_release_date ON game_versions(release_date DESC);

-- ============================================================================
-- ARCADE_GAME_ASSIGNMENTS TABLE
-- Assigns specific game versions to arcades
-- ============================================================================
CREATE TABLE arcade_game_assignments (
    id SERIAL PRIMARY KEY,
    arcade_id INTEGER NOT NULL REFERENCES arcades(id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    assigned_version_id INTEGER NOT NULL REFERENCES game_versions(id) ON DELETE RESTRICT,
    current_version_id INTEGER REFERENCES game_versions(id) ON DELETE SET NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(arcade_id, game_id)  -- Each arcade can have only one assignment per game
);

-- Indexes for frequent queries
CREATE INDEX idx_assignments_arcade_id ON arcade_game_assignments(arcade_id);
CREATE INDEX idx_assignments_game_id ON arcade_game_assignments(game_id);
CREATE INDEX idx_assignments_updated_at ON arcade_game_assignments(updated_at DESC);

-- ============================================================================
-- SNORLAX_VERSIONS TABLE
-- Snorlax APK versions (Quest launcher app)
-- ============================================================================
CREATE TABLE snorlax_versions (
    id SERIAL PRIMARY KEY,
    version VARCHAR(50) UNIQUE NOT NULL,
    gcs_path VARCHAR(512) NOT NULL,  -- Path to APK in GCS
    release_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_current BOOLEAN NOT NULL DEFAULT false,  -- Only one version should be current
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for finding current version
CREATE INDEX idx_snorlax_versions_is_current ON snorlax_versions(is_current);
CREATE INDEX idx_snorlax_versions_release_date ON snorlax_versions(release_date DESC);

-- ============================================================================
-- CONSTRAINTS & COMMENTS
-- ============================================================================

-- Add comments for documentation
COMMENT ON TABLE arcades IS 'Physical VR arcade installations worldwide';
COMMENT ON TABLE games IS 'VR games available in the system';
COMMENT ON TABLE game_versions IS 'Specific versions of games stored in GCS';
COMMENT ON TABLE arcade_game_assignments IS 'Assigns game versions to specific arcades';
COMMENT ON TABLE snorlax_versions IS 'Snorlax APK versions (Quest launcher application)';

COMMENT ON COLUMN arcade_game_assignments.assigned_version_id IS 'Version that should be installed on the arcade';
COMMENT ON COLUMN arcade_game_assignments.current_version_id IS 'Version currently installed (null if not yet downloaded)';
COMMENT ON COLUMN snorlax_versions.is_current IS 'Only one version should have this set to true';
