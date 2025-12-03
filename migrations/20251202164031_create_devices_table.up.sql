-- Add up migration script here
-- CREATE TABLE devices (
--     id INTEGER PRIMARY KEY NOT NULL,
--     serial_number VARCHAR(255) NOT NULL
-- );

CREATE TABLE devices (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    serial_number VARCHAR(255) NOT NULL
);
