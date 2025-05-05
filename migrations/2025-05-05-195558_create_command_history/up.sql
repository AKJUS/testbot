-- Your SQL goes here

CREATE TABLE command_history (
    id SERIAL PRIMARY KEY,
    user VARCHAR NOT NULL,
    command VARCHAR NOT NULL,
    timestamp TIMESTAMP NOT NULL
);
