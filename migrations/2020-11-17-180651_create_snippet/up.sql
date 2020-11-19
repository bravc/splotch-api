-- Your SQL goes here
CREATE TABLE snippets (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
  name VARCHAR NOT NULL,
  timestamp_start INTEGER NOT NULL,
  timestamp_end INTEGER NOT NULL,
  track_uri VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)