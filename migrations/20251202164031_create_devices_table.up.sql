-- Add up migration script here
CREATE TABLE devices (
    id INTEGER PRIMARY KEY NOT NULL,
    serial_number VARCHAR(255) NOT NULL
);
