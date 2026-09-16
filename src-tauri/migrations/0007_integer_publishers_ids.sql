PRAGMA foreign_keys = OFF;
BEGIN IMMEDIATE;

CREATE TABLE publishers_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

INSERT INTO publishers_new (name)
SELECT name
FROM publishers
ORDER BY rowid;

CREATE TEMP TABLE publishers_id_map AS
SELECT legacy.id AS old_id, numbered.new_id
FROM (
  SELECT id, ROW_NUMBER() OVER (ORDER BY rowid) AS position
  FROM publishers
) legacy
JOIN (
  SELECT id AS new_id, ROW_NUMBER() OVER (ORDER BY id) AS position
  FROM publishers_new
) numbered ON numbered.position = legacy.position;

DROP TABLE publishers;
ALTER TABLE publishers_new RENAME TO publishers;

CREATE TABLE series_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT,
  publisher_id INTEGER,
  start_year INTEGER,
  end_year INTEGER,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (publisher_id) REFERENCES publishers(id)
);

INSERT INTO series_new (
  id,
  title,
  description,
  publisher_id,
  start_year,
  end_year,
  notes,
  created_at,
  updated_at
)
SELECT
  s.id,
  s.title,
  s.description,
  id_map.new_id,
  s.start_year,
  s.end_year,
  s.notes,
  s.created_at,
  s.updated_at
FROM series s
LEFT JOIN publishers_id_map id_map ON id_map.old_id = s.publisher_id
ORDER BY s.id;

DROP TABLE series;
ALTER TABLE series_new RENAME TO series;

DROP TABLE publishers_id_map;

COMMIT;
PRAGMA foreign_keys = ON;
