-- Initial schema for comics-list (simplified: series + publishers + entries)
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS publishers (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  notes TEXT
);

CREATE TABLE IF NOT EXISTS series (
  id TEXT PRIMARY KEY,
  -- title is the franchise/title (e.g. "Spawn")
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

CREATE TABLE IF NOT EXISTS entries (
  id TEXT PRIMARY KEY,
  series_id TEXT NOT NULL,
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
  FOREIGN KEY (series_id) REFERENCES series(id)
);

