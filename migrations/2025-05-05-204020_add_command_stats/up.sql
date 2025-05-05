-- Your SQL goes here

CREATE TABLE command_stats (
    id SERIAL PRIMARY KEY,
    command VARCHAR NOT NULL,
    arguments TEXT NOT NULL,
    count INTEGER NOT NULL DEFAULT 1,
    last_used TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(command, arguments)
);
