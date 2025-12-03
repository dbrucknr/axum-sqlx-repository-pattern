-- Add down migration script here
-- SQLite Dialect (original)
-- DROP TABLE devices;

-- CREATE TABLE IF NOT EXISTS devices (
--     id INTEGER PRIMARY KEY NOT NULL,
--     serial_number VARCHAR(255) NOT NULL
-- );
-- Updated to use Postgres Dialect
DROP TABLE IF EXISTS devices;

CREATE TABLE IF NOT EXISTS devices (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    serial_number VARCHAR(255) NOT NULL
);
