-- ============================================================================
-- DATABASE RESET SCRIPT
-- WARNING: This will DROP ALL TABLES and recreate them with the new schema
-- All existing data will be PERMANENTLY LOST
-- ============================================================================

-- Drop all tables (in reverse order of dependencies)
DROP TABLE IF EXISTS sensors CASCADE;
DROP TABLE IF EXISTS gyros_versions CASCADE;
DROP TABLE IF EXISTS game_version_channels CASCADE;
DROP TABLE IF EXISTS arcade_game_assignments CASCADE;
DROP TABLE IF EXISTS game_versions CASCADE;
DROP TABLE IF EXISTS games CASCADE;
DROP TABLE IF EXISTS snorlax_versions CASCADE;
DROP TABLE IF EXISTS arcades CASCADE;
DROP TABLE IF EXISTS customers CASCADE;
DROP TABLE IF EXISTS release_channels CASCADE;

-- Drop types
DROP TYPE IF EXISTS release_channel CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS ensure_single_current_gyros() CASCADE;
DROP FUNCTION IF EXISTS ensure_single_current_snorlax() CASCADE;
DROP FUNCTION IF EXISTS ensure_one_game_version_per_channel() CASCADE;

-- ============================================================================
-- RELEASE CHANNELS TABLE
-- Dynamic table for managing release channels (can add/remove channels)
-- ============================================================================
CREATE TABLE release_channels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Insert default channels
INSERT INTO release_channels (name, description) VALUES
    ('production', 'Stable production releases for customer arcades'),
    ('test', 'Pre-release versions for internal testing'),
    ('development', 'Infrastructure development channel. should only be used by B3n00n');

CREATE INDEX idx_release_channels_name ON release_channels(name);

COMMENT ON TABLE release_channels IS 'Release channels for game version distribution';

-- ============================================================================
-- CUSTOMERS TABLE
-- Represents customers who own arcade installations
-- ============================================================================
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone_number VARCHAR(50),
    email VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customers_name ON customers(name);

COMMENT ON TABLE customers IS 'Customers who own arcade installations';
COMMENT ON COLUMN customers.name IS 'Customer display name';
COMMENT ON COLUMN customers.phone_number IS 'Contact phone number (optional)';
COMMENT ON COLUMN customers.email IS 'Contact email address (optional)';

-- ============================================================================
-- ARCADES TABLE
-- Represents physical VR arcade installations
-- ============================================================================
CREATE TABLE arcades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    machine_id VARCHAR(255) UNIQUE NOT NULL,  -- Machine ID format: 32-char hex string
    status VARCHAR(50) NOT NULL DEFAULT 'active',  -- active, inactive, maintenance
    channel_id INTEGER NOT NULL REFERENCES release_channels(id) ON DELETE RESTRICT DEFAULT 1,  -- FK to release_channels (defaults to production)
    customer_id INTEGER REFERENCES customers(id) ON DELETE RESTRICT,  -- FK to customers (nullable for unassigned arcades)
    installed_games JSONB DEFAULT '{}'::jsonb,  -- Map of game_id to version: {"1": "v0.8.2", "2": "v1.2.0"}
    last_seen_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for frequent lookups
CREATE INDEX idx_arcades_machine_id ON arcades(machine_id);
CREATE INDEX idx_arcades_status ON arcades(status);
CREATE INDEX idx_arcades_channel_id ON arcades(channel_id);
CREATE INDEX idx_arcades_customer_id ON arcades(customer_id);

COMMENT ON TABLE arcades IS 'Physical VR arcade installations worldwide';
COMMENT ON COLUMN arcades.machine_id IS 'Unique machine identifier (stable across network adapter changes)';
COMMENT ON COLUMN arcades.channel_id IS 'Release channel for this arcade (determines which game versions are available)';
COMMENT ON COLUMN arcades.customer_id IS 'Customer who owns this arcade (nullable for unassigned arcades)';
COMMENT ON COLUMN arcades.installed_games IS 'JSON map of installed games: {"game_id": "version_string"}';

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
-- ARCADE_GAME_ASSIGNMENTS TABLE
-- Explicit game assignments per arcade. Arcade only gets assigned games.
-- ============================================================================
CREATE TABLE arcade_game_assignments (
    arcade_id INTEGER NOT NULL REFERENCES arcades(id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (arcade_id, game_id)
);

CREATE INDEX idx_arcade_game_assignments_arcade_id ON arcade_game_assignments(arcade_id);
CREATE INDEX idx_arcade_game_assignments_game_id ON arcade_game_assignments(game_id);

COMMENT ON TABLE arcade_game_assignments IS 'Explicit game assignments per arcade. Arcade only receives games listed here.';

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
-- GAME_VERSION_CHANNELS TABLE
-- Junction table: Which channels is each version published to?
-- A version can be published to multiple channels simultaneously
-- ============================================================================
CREATE TABLE game_version_channels (
    version_id INTEGER NOT NULL REFERENCES game_versions(id) ON DELETE CASCADE,
    channel_id INTEGER NOT NULL REFERENCES release_channels(id) ON DELETE CASCADE,
    published_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (version_id, channel_id)
);

-- Indexes for frequent queries
CREATE INDEX idx_game_version_channels_version_id ON game_version_channels(version_id);
CREATE INDEX idx_game_version_channels_channel_id ON game_version_channels(channel_id);

COMMENT ON TABLE game_version_channels IS 'Junction table mapping versions to release channels (many-to-many)';

-- Trigger function to ensure only one version of a game per channel
CREATE OR REPLACE FUNCTION ensure_one_game_version_per_channel()
RETURNS TRIGGER AS $$
DECLARE
    new_game_id INTEGER;
    conflicting_count INTEGER;
BEGIN
    -- Get the game_id for the version being published
    SELECT game_id INTO new_game_id
    FROM game_versions
    WHERE id = NEW.version_id;

    -- Check if another version of this game already exists on this channel
    SELECT COUNT(*) INTO conflicting_count
    FROM game_version_channels gvc
    JOIN game_versions gv ON gvc.version_id = gv.id
    WHERE gvc.channel_id = NEW.channel_id
      AND gv.game_id = new_game_id
      AND gvc.version_id != NEW.version_id;

    IF conflicting_count > 0 THEN
        -- Remove the old version(s) automatically
        DELETE FROM game_version_channels gvc
        USING game_versions gv
        WHERE gvc.version_id = gv.id
          AND gvc.channel_id = NEW.channel_id
          AND gv.game_id = new_game_id
          AND gvc.version_id != NEW.version_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER game_version_per_channel_trigger
    BEFORE INSERT ON game_version_channels
    FOR EACH ROW
    EXECUTE FUNCTION ensure_one_game_version_per_channel();

COMMENT ON FUNCTION ensure_one_game_version_per_channel() IS 'Automatically removes old versions of a game from a channel when publishing a new version';

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
-- SENSORS TABLE
-- Tracks individual sensors deployed in arcades
-- ============================================================================
CREATE TABLE sensors (
    id SERIAL PRIMARY KEY,
    serial_number VARCHAR(255) UNIQUE NOT NULL,
    mac_address VARCHAR(50),
    firmware_version VARCHAR(50),
    arcade_id INTEGER REFERENCES arcades(id) ON DELETE SET NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sensors_serial_number ON sensors(serial_number);
CREATE INDEX idx_sensors_arcade_id ON sensors(arcade_id);

COMMENT ON TABLE sensors IS 'Individual sensors deployed in arcades, reported by Arceus after firmware upload';
COMMENT ON COLUMN sensors.serial_number IS 'Unique sensor serial number read from the device';
COMMENT ON COLUMN sensors.mac_address IS 'BLE MAC address of the sensor';
COMMENT ON COLUMN sensors.firmware_version IS 'Currently installed firmware version';
COMMENT ON COLUMN sensors.arcade_id IS 'Arcade this sensor belongs to (matched by machine_id)';

-- ============================================================================
-- SCRIPT COMPLETE
-- All tables have been recreated with the new schema
-- ============================================================================
