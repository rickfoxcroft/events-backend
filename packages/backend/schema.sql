CREATE TABLE IF NOT EXISTS venues (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    capacity INTEGER NOT NULL,
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
