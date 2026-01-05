-- Alakazam Database Schema

-- VR Arcade locations
CREATE TABLE arcades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    api_key VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(50) DEFAULT 'active',
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Available games
CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Game versions with build artifacts
CREATE TABLE game_versions (
    id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    version VARCHAR(50) NOT NULL,

    -- GCS paths
    pc_build_path VARCHAR(500) NOT NULL,
    quest_apk_path VARCHAR(500) NOT NULL,
    data_content_path VARCHAR(500) NOT NULL,

    release_date TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(game_id, version)
);

-- Which version each arcade should run
CREATE TABLE arcade_game_assignments (
    id SERIAL PRIMARY KEY,
    arcade_id INTEGER NOT NULL REFERENCES arcades(id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    assigned_version_id INTEGER NOT NULL REFERENCES game_versions(id),
    current_version_id INTEGER REFERENCES game_versions(id),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(arcade_id, game_id)
);

-- Indexes
CREATE INDEX idx_arcades_api_key ON arcades(api_key);
CREATE INDEX idx_game_versions_game_id ON game_versions(game_id);
CREATE INDEX idx_assignments_arcade ON arcade_game_assignments(arcade_id);
