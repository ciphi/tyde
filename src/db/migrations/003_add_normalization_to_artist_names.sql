PRAGMA foreign_keys = OFF;

BEGIN;

-- Add normalization column
CREATE TABLE artist_names_new (
    id INTEGER PRIMARY KEY,
    artist_id INTEGER,
    name TEXT NOT NULL,
    normalized_name TEXT,
    kind TEXT NULL,
    locale TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

INSERT INTO artist_names_new (
    id, artist_id, name, kind, locale, created_at, updated_at
)
SELECT
    id,
    artist_id,
    name,
    kind,
    locale,
    created_at,
    updated_at
FROM artist_names;

DROP TABLE artist_names;
ALTER TABLE artist_names_new RENAME TO artist_names;

COMMIT;

PRAGMA foreign_keys = ON;
