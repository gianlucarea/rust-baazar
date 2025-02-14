-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT CHECK (role IN ('Admin', 'User')) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE weapon_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE material_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE weapons (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    weight DECIMAL(10,2) NOT NULL CHECK (weight > 0), -- Weight in kg
    origin VARCHAR(100) NOT NULL,  -- Region or civilization
    first_use_year INT CHECK (first_use_year <= 1750), -- Year of first known use
    type_id INT NOT NULL,
    material_id INT NOT NULL,
    FOREIGN KEY (type_id) REFERENCES weapon_type(id) ON DELETE CASCADE,
    FOREIGN KEY (material_id) REFERENCES material_type(id) ON DELETE CASCADE
);

-- Insert materials
INSERT INTO material_type (name) VALUES
('Steel'),
('Iron'),
('Bronze'),
('Wood'),
('Horn'),
('Stone'),
('Composite');

-- Insert expanded weapon types (pre-1750)
INSERT INTO weapon_type (name) VALUES
('Sword'),
('Dagger'),
('Bow'),
('Crossbow'),
('Spear'),
('Axe'),
('Mace'),
('Halberd'),
('Flail'),
('Warhammer'),
('Polearm'),
('Scimitar'),
('Katana'),
('Naginata'),
('Flintlock Firearm'),
('Cannon'),
('Trebuchet'),
('Ballista'),
('Shield');

INSERT INTO weapons (name, weight, origin, first_use_year, type_id, material_id) VALUES
-- Swords
('Longsword', 1.5, 'Europe', 1000, 1, 1),
('Gladius', 1.0, 'Rome', -300, 1, 2),
('Rapier', 1.2, 'Spain', 1500, 1, 1),
('Claymore', 2.5, 'Scotland', 1400, 1, 1),
('Scimitar', 1.3, 'Middle East', 900, 12, 1),
('Katana', 1.1, 'Japan', 1400, 13, 1),

-- Daggers
('Kris Dagger', 0.6, 'Southeast Asia', 1300, 2, 2),
('Stiletto Dagger', 0.4, 'Italy', 1500, 2, 1),

-- Bows & Crossbows
('English Longbow', 0.5, 'England', 1250, 3, 4),
('Mongol Recurve Bow', 0.6, 'Mongolia', 1100, 3, 6),
('Chinese Repeating Crossbow', 2.0, 'China', -400, 4, 4),

-- Spears & Polearms
('Pike', 3.5, 'Europe', 1400, 5, 1),
('Greek Dory', 2.0, 'Greece', -700, 5, 3),
('Naginata', 3.0, 'Japan', 800, 14, 1),
('Halberd', 3.0, 'Europe', 1300, 8, 1),

-- Axes & Maces
('Viking War Axe', 2.5, 'Scandinavia', 800, 6, 2),
('Double-headed Battle Axe', 3.5, 'Persia', 1100, 6, 1),
('Flanged Mace', 2.0, 'Medieval Europe', 1200, 7, 2),
('Indian Gurz', 2.2, 'India', 1500, 7, 3),

-- Flails & Warhammers
('Peasant Flail', 3.0, 'Europe', 1300, 9, 4),
('Warhammer', 2.5, 'Holy Roman Empire', 1400, 10, 1),

-- Firearms & Siege Weapons
('Flintlock Musket', 4.5, 'France', 1610, 15, 1),
('Ottoman Cannon', 900.0, 'Ottoman Empire', 1450, 16, 3),
('Trebuchet', 12000.0, 'Europe', 1100, 17, 4),
('Roman Ballista', 400.0, 'Rome', -200, 18, 4),

-- Shields (defensive weapons)
('Roman Scutum', 5.0, 'Rome', -300, 19, 3),
('Viking Round Shield', 4.0, 'Scandinavia', 900, 19, 4),
('Heater Shield', 6.0, 'Europe', 1200, 19, 1);
