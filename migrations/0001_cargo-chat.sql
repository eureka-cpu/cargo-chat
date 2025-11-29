CREATE TABLE IF NOT EXISTS index_state (
    id INTEGER PRIMARY KEY,
    metadata_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS file_signatures (
    path TEXT PRIMARY KEY,
    modified INTEGER NOT NULL,
    size INTEGER NOT NULL
);
