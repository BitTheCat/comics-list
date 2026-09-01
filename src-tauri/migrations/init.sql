-- Initial schema for comics-list
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS works (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS publishers (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  notes TEXT
);

CREATE TABLE IF NOT EXISTS series (
  id TEXT PRIMARY KEY,
  work_id TEXT NOT NULL,
  publisher_id TEXT,
  title TEXT,
  start_year INTEGER,
  end_year INTEGER,
  notes TEXT,
  FOREIGN KEY (work_id) REFERENCES works(id),
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

