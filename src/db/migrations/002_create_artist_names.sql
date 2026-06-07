PRAGMA foreign_keys = OFF;

BEGIN;

-- New table to store artist names
CREATE TABLE artist_names (
    id INTEGER PRIMARY KEY,
    artist_id INTEGER,
    name TEXT NOT NULL,
    kind TEXT NULL,
    locale TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

-- Create new updated artist table
CREATE TABLE artists_new (
    id INTEGER PRIMARY KEY,
    primary_name_id INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Transfer names from old table to artist_names table
INSERT INTO artist_names (artist_id, name, created_at, updated_at)
SELECT
    id,
    name,
    created_at,
    updated_at
FROM artists;

-- Set primary ID
INSERT INTO artists_new (primary_name_id, created_at, updated_at)
SELECT
    id,
    created_at,
    updated_at
FROM artist_names;

-- Drop and rename
DROP TABLE artists;
ALTER TABLE artists_new RENAME TO artists;

COMMIT;

PRAGMA foreign_keys = ON;
