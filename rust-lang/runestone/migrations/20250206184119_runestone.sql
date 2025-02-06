-- Add migration script here
CREATE TABLE runestones (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    material VARCHAR(100),
    inscription TEXT,
    magic_type VARCHAR(100),
    power_level INT,
    discovered_on DATE,
    location VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);