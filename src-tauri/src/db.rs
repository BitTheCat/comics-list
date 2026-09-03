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
    Ok(conn)
}

#[derive(Debug, Serialize)]
pub struct SeriesRow {
    pub id: String,
    pub title: String,
    pub series_title: Option<String>,
    pub publisher_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn list_series(conn: &Connection) -> Result<Vec<SeriesRow>> {
    let mut stmt = conn.prepare("SELECT id, title, series_title, publisher_id, start_year, end_year, notes, created_at, updated_at FROM series ORDER BY title COLLATE NOCASE")?;
    let rows = stmt.query_map([], |r| {
        Ok(SeriesRow {
            id: r.get(0)?,
            title: r.get(1)?,
            series_title: r.get(2).ok(),
            publisher_id: r.get(3).ok(),
            start_year: r.get(4).ok(),
            end_year: r.get(5).ok(),
            notes: r.get(6).ok(),
            created_at: r.get(7)?,
            updated_at: r.get(8)?,
        })
    })?;

    let mut res = Vec::new();
    for s in rows {
        res.push(s?);
    }
    Ok(res)
}

pub fn create_series(conn: &Connection, id: &str, title: &str, series_title: Option<&str>, publisher_id: Option<&str>, start_year: Option<i32>, end_year: Option<i32>, notes: Option<&str>, now: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO series (id, title, series_title, publisher_id, start_year, end_year, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
        params![id, title, series_title, publisher_id, start_year, end_year, notes, now],
    )?;
    Ok(())
}

