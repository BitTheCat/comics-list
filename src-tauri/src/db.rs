use rusqlite::{params, Connection, Result};
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

#[derive(Debug)]
pub struct WorkRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn list_works(conn: &Connection) -> Result<Vec<WorkRow>> {
    let mut stmt = conn.prepare("SELECT id, title, description, created_at, updated_at FROM works ORDER BY title COLLATE NOCASE")?;
    let rows = stmt.query_map([], |r| {
        Ok(WorkRow {
            id: r.get(0)?,
            title: r.get(1)?,
            description: r.get(2).ok(),
            created_at: r.get(3)?,
            updated_at: r.get(4)?,
        })
    })?;

    let mut res = Vec::new();
    for w in rows {
        res.push(w?);
    }
    Ok(res)
}

pub fn create_work(conn: &Connection, id: &str, title: &str, description: Option<&str>, now: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO works (id, title, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
        params![id, title, description, now],
    )?;
    Ok(())
}

