-- Add migration script here
CREATE TABLE users (
  user_id VARCHAR(36) NOT NULL UNIQUE,
  username VARCHAR(255) NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  PRIMARY KEY (user_id)
);
