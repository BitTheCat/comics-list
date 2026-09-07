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
    run_migrations(&conn)?;
    Ok(conn)
}

struct Migration {
    version: i64,
    name: &'static str,
    sql: &'static str,
    should_run: fn(&Connection) -> Result<bool>,
}

fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    let migrations = [
        Migration {
            version: 1,
            name: "initial schema",
            sql: include_str!("../migrations/0001_initial_schema.sql"),
            should_run: |_| Ok(true),
        },
        Migration {
            version: 2,
            name: "rename series title to description",
            sql: include_str!("../migrations/0002_rename_series_title_to_description.sql"),
            should_run: |connection| column_exists(connection, "series", "series_title"),
        },
        Migration {
            version: 3,
            name: "use integer series identifiers",
            sql: include_str!("../migrations/0003_integer_series_ids.sql"),
            should_run: |connection| {
                let id_type: String = connection.query_row(
                    "SELECT type FROM pragma_table_info('series') WHERE name = 'id'",
                    [],
                    |row| row.get(0),
                )?;
                Ok(!id_type.eq_ignore_ascii_case("INTEGER"))
            },
        },
        Migration {
            version: 4,
            name: "add entry condition",
            sql: include_str!("../migrations/0004_add_entry_condition.sql"),
            should_run: |connection| Ok(!column_exists(connection, "entries", "condition")?),
        },
    ];

    for migration in migrations {
        let was_applied: bool = conn.query_row(
            "SELECT EXISTS(
                SELECT 1 FROM schema_migrations WHERE version = ?1
            )",
            [migration.version],
            |row| row.get(0),
        )?;
        if was_applied {
            continue;
        }

        if (migration.should_run)(conn)? {
            conn.execute_batch(migration.sql)?;
        }
        conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            params![migration.version, migration.name],
        )?;
    }

    Ok(())
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    conn.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM pragma_table_info(?1) WHERE name = ?2
        )",
        params![table, column],
        |row| row.get(0),
    )
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
