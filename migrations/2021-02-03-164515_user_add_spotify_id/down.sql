-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN spotify_id;
ALTER TABLE users DROP COLUMN profile_url;