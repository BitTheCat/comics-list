PRAGMA foreign_keys = OFF;
BEGIN IMMEDIATE;

CREATE TABLE series_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT,
  publisher_id TEXT,
  start_year INTEGER,
  end_year INTEGER,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (publisher_id) REFERENCES publishers(id)
);

INSERT INTO series_new (
  title, description, publisher_id, start_year, end_year, notes, created_at, updated_at
)
SELECT
  title, description, publisher_id, start_year, end_year, notes, created_at, updated_at
FROM series
ORDER BY rowid;

CREATE TEMP TABLE series_id_map AS
SELECT legacy.id AS old_id, numbered.new_id
FROM (
  SELECT id, ROW_NUMBER() OVER (ORDER BY rowid) AS position
  FROM series
) legacy
JOIN (
  SELECT id AS new_id, ROW_NUMBER() OVER (ORDER BY id) AS position
  FROM series_new
) numbered ON numbered.position = legacy.position;

CREATE TABLE entries_new (
  id TEXT PRIMARY KEY,
  series_id INTEGER NOT NULL,
  issue_number TEXT NOT NULL,
  volume TEXT,
  title TEXT,
  description TEXT,
  pub_year INTEGER,
  pub_month INTEGER,
  cover_path TEXT,
  cover_url TEXT,
  tags TEXT,
  read_status TEXT,
  rating INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (series_id) REFERENCES series_new(id)
);

INSERT INTO entries_new (
  id, series_id, issue_number, volume, title, description, pub_year, pub_month,
  cover_path, cover_url, tags, read_status, rating, created_at, updated_at
)
SELECT
  entry.id, id_map.new_id, entry.issue_number, entry.volume, entry.title,
  entry.description, entry.pub_year, entry.pub_month, entry.cover_path,
  entry.cover_url, entry.tags, entry.read_status, entry.rating,
  entry.created_at, entry.updated_at
FROM entries entry
JOIN series_id_map id_map ON id_map.old_id = entry.series_id;

DROP TABLE entries;
DROP TABLE series;
ALTER TABLE series_new RENAME TO series;
ALTER TABLE entries_new RENAME TO entries;
CREATE INDEX idx_entries_series_id ON entries(series_id);
DROP TABLE series_id_map;

COMMIT;
PRAGMA foreign_keys = ON;
