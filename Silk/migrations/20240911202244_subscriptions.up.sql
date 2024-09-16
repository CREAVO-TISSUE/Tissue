-- Add up migration script here
CREATE TABLE IF NOT EXISTS subscriptions
(
    id INTEGER PRIMARY KEY ASC,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
