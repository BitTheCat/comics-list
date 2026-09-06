use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::path::PathBuf;

pub fn get_app_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut p| {
        p.push("comics-list");
        p
    })
}

pub fn db_path() -> Option<PathBuf> {
    get_app_dir().map(|mut d| {
        std::fs::create_dir_all(&d).ok();
        d.push("comics-list.db");
        d
    })
}

pub fn init_db() -> Result<Connection> {
    let path = db_path().expect("could not determine app data directory");
    let conn = Connection::open(path)?;
    // Execute migration bundled with the binary
    let sql = include_str!("../migrations/init.sql");
    conn.execute_batch(sql)?;
    migrate_series_description(&conn)?;
    migrate_series_id_to_integer(&conn)?;
    Ok(conn)
}

fn migrate_series_description(conn: &Connection) -> Result<()> {
    let has_series_title: bool = conn.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM pragma_table_info('series') WHERE name = 'series_title'
        )",
        [],
        |row| row.get(0),
    )?;
    let has_description: bool = conn.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM pragma_table_info('series') WHERE name = 'description'
        )",
        [],
        |row| row.get(0),
    )?;

    if has_series_title && !has_description {
        conn.execute(
            "ALTER TABLE series RENAME COLUMN series_title TO description",
            [],
        )?;
    }

    Ok(())
}

fn migrate_series_id_to_integer(conn: &Connection) -> Result<()> {
    let id_type: String = conn.query_row(
        "SELECT type FROM pragma_table_info('series') WHERE name = 'id'",
        [],
        |row| row.get(0),
    )?;
    if id_type.eq_ignore_ascii_case("INTEGER") {
        return Ok(());
    }

    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
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
        PRAGMA foreign_keys = ON;",
    )?;

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct SeriesRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub publisher_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub entries_count: i64,
}

pub fn list_series(conn: &Connection) -> Result<Vec<SeriesRow>> {
    let mut stmt = conn.prepare(
        "SELECT
            s.id,
            s.title,
            s.description,
            s.publisher_id,
            s.start_year,
            s.end_year,
            s.notes,
            s.created_at,
            s.updated_at,
            (
                SELECT COUNT(*)
                FROM entries e
                WHERE e.series_id = s.id
            ) AS entries_count
        FROM series s
        ORDER BY s.title COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(SeriesRow {
            id: r.get(0)?,
            title: r.get(1)?,
            description: r.get(2).ok(),
            publisher_id: r.get(3).ok(),
            start_year: r.get(4).ok(),
            end_year: r.get(5).ok(),
            notes: r.get(6).ok(),
            created_at: r.get(7)?,
            updated_at: r.get(8)?,
            entries_count: r.get(9)?,
        })
    })?;

    let mut res = Vec::new();
    for s in rows {
        res.push(s?);
    }
    Ok(res)
}

pub fn create_series(
    conn: &Connection,
    title: &str,
    description: Option<&str>,
    publisher_id: Option<&str>,
    start_year: Option<i32>,
    end_year: Option<i32>,
    notes: Option<&str>,
    now: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO series (title, description, publisher_id, start_year, end_year, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
        params![title, description, publisher_id, start_year, end_year, notes, now],
    )?;
    Ok(conn.last_insert_rowid())
}
