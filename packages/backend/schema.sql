CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    provider_id TEXT NOT NULL,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    avatar_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(provider_id),
    UNIQUE(email)
);

CREATE TABLE IF NOT EXISTS venues (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    capacity INTEGER NOT NULL,
    price_per_hour INTEGER NOT NULL DEFAULT 0,
    owner_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS venue_images (
    id TEXT PRIMARY KEY,
    venue_id TEXT NOT NULL,
    url TEXT NOT NULL,
    FOREIGN KEY (venue_id) REFERENCES venues(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    venue_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    FOREIGN KEY (venue_id) REFERENCES venues(id)
);
