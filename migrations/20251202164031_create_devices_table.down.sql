-- Add down migration script here
DROP TABLE devices;

CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY NOT NULL,
    serial_number VARCHAR(255) NOT NULL
);
