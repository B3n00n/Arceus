-- ============================================================================
-- DATABASE RESET SCRIPT
-- WARNING: This will DROP ALL TABLES and recreate them with the new schema
-- All existing data will be PERMANENTLY LOST
-- ============================================================================

-- Drop all tables (in reverse order of dependencies)
DROP TABLE IF EXISTS gyros_versions CASCADE;
DROP TABLE IF EXISTS arcade_game_assignments CASCADE;
DROP TABLE IF EXISTS game_versions CASCADE;
DROP TABLE IF EXISTS games CASCADE;
DROP TABLE IF EXISTS snorlax_versions CASCADE;
DROP TABLE IF EXISTS arcades CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS ensure_single_current_gyros() CASCADE;
DROP FUNCTION IF EXISTS ensure_single_current_snorlax() CASCADE;

-- ============================================================================
-- ARCADES TABLE (UPDATED: mac_address -> machine_id)
-- Represents physical VR arcade installations
-- ============================================================================
CREATE TABLE arcades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    machine_id VARCHAR(255) UNIQUE NOT NULL,  -- Machine ID format: 32-char hex string
    status VARCHAR(50) NOT NULL DEFAULT 'active',  -- active, inactive, maintenance
    last_seen_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for frequent machine ID lookups
CREATE INDEX idx_arcades_machine_id ON arcades(machine_id);
CREATE INDEX idx_arcades_status ON arcades(status);

COMMENT ON TABLE arcades IS 'Physical VR arcade installations worldwide';
COMMENT ON COLUMN arcades.machine_id IS 'Unique machine identifier (stable across network adapter changes)';

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

COMMENT ON TABLE games IS 'VR games available in the system';

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

COMMENT ON TABLE game_versions IS 'Specific versions of games stored in GCS';

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

COMMENT ON TABLE arcade_game_assignments IS 'Assigns game versions to specific arcades';
COMMENT ON COLUMN arcade_game_assignments.assigned_version_id IS 'Version that should be installed on the arcade';
COMMENT ON COLUMN arcade_game_assignments.current_version_id IS 'Version currently installed (null if not yet downloaded)';

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

COMMENT ON TABLE snorlax_versions IS 'Snorlax APK versions (Quest launcher application)';
COMMENT ON COLUMN snorlax_versions.is_current IS 'Only one version should have this set to true';

-- Trigger to ensure only one current Snorlax version
CREATE OR REPLACE FUNCTION ensure_single_current_snorlax()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_current = true THEN
        UPDATE snorlax_versions SET is_current = false WHERE id != NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER snorlax_version_current_trigger
    AFTER INSERT OR UPDATE ON snorlax_versions
    FOR EACH ROW
    WHEN (NEW.is_current = true)
    EXECUTE FUNCTION ensure_single_current_snorlax();

-- ============================================================================
-- GYROS_VERSIONS TABLE
-- Gyros firmware versions (sensor firmware)
-- ============================================================================
CREATE TABLE gyros_versions (
    id SERIAL PRIMARY KEY,
    version VARCHAR(50) UNIQUE NOT NULL,
    gcs_path VARCHAR(512) NOT NULL,
    release_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_current BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for finding current version
CREATE INDEX idx_gyros_versions_is_current ON gyros_versions(is_current);
CREATE INDEX idx_gyros_versions_release_date ON gyros_versions(release_date DESC);

COMMENT ON TABLE gyros_versions IS 'Gyros firmware versions (sensor firmware)';

-- Trigger to ensure only one current Gyros version
CREATE OR REPLACE FUNCTION ensure_single_current_gyros()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_current = true THEN
        UPDATE gyros_versions SET is_current = false WHERE id != NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER gyros_version_current_trigger
    AFTER INSERT OR UPDATE ON gyros_versions
    FOR EACH ROW
    WHEN (NEW.is_current = true)
    EXECUTE FUNCTION ensure_single_current_gyros();

-- ============================================================================
-- SCRIPT COMPLETE
-- All tables have been recreated with the new schema
-- ============================================================================
