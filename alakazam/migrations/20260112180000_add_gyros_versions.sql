-- Add gyros_versions table for sensor firmware management
-- Similar to snorlax_versions but for Gyros firmware

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

-- Ensure only one current version at a time
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
