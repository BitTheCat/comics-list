PRAGMA foreign_keys = OFF;
BEGIN IMMEDIATE;

CREATE TABLE entries_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  series_id INTEGER NOT NULL,
  issue_number TEXT NOT NULL,
  title TEXT,
  pub_year INTEGER,
  pub_month INTEGER,
  cover_path TEXT,
  tags TEXT,
  condition TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (series_id) REFERENCES series(id)
);

INSERT INTO entries_new (
  series_id,
  issue_number,
  title,
  pub_year,
  pub_month,
  cover_path,
  tags,
  condition,
  created_at,
  updated_at
)
SELECT
  series_id,
  issue_number,
  title,
  pub_year,
  pub_month,
  cover_path,
  tags,
  condition,
  created_at,
  updated_at
FROM entries
ORDER BY rowid;

DROP TABLE entries;
ALTER TABLE entries_new RENAME TO entries;
CREATE INDEX idx_entries_series_id ON entries(series_id);

COMMIT;
PRAGMA foreign_keys = ON;
