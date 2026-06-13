PRAGMA foreign_keys = OFF;

BEGIN;

ALTER TABLE artist_names RENAME COLUMN locale TO lang;

COMMIT;

PRAGMA foreign_keys = ON;
