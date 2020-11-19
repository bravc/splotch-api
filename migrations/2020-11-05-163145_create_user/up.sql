-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  pass_hash VARCHAR NOT NULL,
  spotify_refresh VARCHAR,
  profile_pic VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)