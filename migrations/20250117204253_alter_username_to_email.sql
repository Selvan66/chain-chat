-- Add migration script here
ALTER TABLE users RENAME COLUMN username TO email;
UPDATE users SET email = CONCAT(email, '@mail.com');
