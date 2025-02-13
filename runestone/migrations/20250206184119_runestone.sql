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

INSERT INTO runestones (name, material, inscription, magic_type, power_level, discovered_on, location)
VALUES 
    ('Ancient Rune of Fire', 'Obsidian', 'Flames of the Eternal', 'Fire', 9, '2025-02-01', 'Volcanic Cavern'),
    ('Rune of Eternal Light', 'Marble', 'Glows with celestial energy', 'Light', 8, '2024-12-10', 'Ancient Temple'),
    ('Shadow Rune', 'Obsidian', 'Absorbs surrounding light', 'Darkness', 7, '2023-06-21', 'Forgotten Crypt'),
    ('Stormcaller Rune', 'Granite', 'Crackles with electricity', 'Lightning', 9, '2022-08-15', 'Floating Isles'),
    ('Frost Rune', 'Sapphire', 'Chills the air around it', 'Ice', 6, '2021-01-30', 'Frozen Cavern'),
    ('Rune of the Earthshaker', 'Basalt', 'Vibrates with seismic power', 'Earth', 10, '2020-11-05', 'Deep Chasm'),
    ('Inferno Rune', 'Ruby', 'Burns with an eternal flame', 'Fire', 9, '2025-03-14', 'Magma Chamber'),
    ('Celestial Rune', 'Quartz', 'Radiates divine energy', 'Holy', 10, '2019-07-22', 'Sky Sanctuary'),
    ('Abyssal Rune', 'Onyx', 'Emits whispers from the void', 'Darkness', 8, '2018-09-09', 'Sunken Ruins'),
    ('Windborne Rune', 'Turquoise', 'Moves slightly in the breeze', 'Air', 7, '2023-05-18', 'High Cliffs'),
    ('Tidal Rune', 'Pearl', 'Feels damp to the touch', 'Water', 6, '2022-10-03', 'Lost Lagoon');
