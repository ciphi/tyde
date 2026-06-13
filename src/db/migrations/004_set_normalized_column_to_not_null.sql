PRAGMA foreign_keys = OFF;

BEGIN;

-- Add normalization column
ALTER TABLE artist_names ALTER COLUMN normalized_name SET NOT NULL ;

COMMIT ;

PRAGMA foreign_keys = ON ;
