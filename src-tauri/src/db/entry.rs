use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EntryRow { 
    pub id: i64,
    pub series_id: i64,
    pub issue_number: String,
    pub title: Option<String>,
    pub pub_year: Option<i32>,
    pub pub_month: Option<i32>,
    pub cover_path: Option<String>,
    pub tags: Option<String>,
    pub condition: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn list_entries(conn: &Connection) -> Result<Vec<EntryRow>> {
    let mut stmt = conn.prepare(
        "SELECT
            e.id,
            e.series_id,
            e.issue_number,
            e.title,
            e.pub_year,
            e.pub_month,
            e.cover_path,
            e.tags,
            e.condition,
            e.created_at,
            e.updated_at
        FROM entries e
        ORDER BY e.issue_number COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(EntryRow {
            id: r.get(0)?,
            series_id: r.get(1)?,
            issue_number: r.get(2)?,
            title: r.get(3).ok(),
            pub_year: r.get(4).ok(),
            pub_month: r.get(5).ok(),
            cover_path: r.get(6).ok(),
            tags: r.get(7).ok(),
            condition: r.get(8).ok(),
            created_at: r.get(10)?,
            updated_at: r.get(11)?,
        })
    })?;

    let mut res = Vec::new();
    for s in rows {
        res.push(s?);
    }
    Ok(res)
}

pub fn create_entry(
    conn: &Connection,
    series_id: &i64,
    issue_number: &str,
    title: Option<&str>,
    pub_year: Option<i32>,
    pub_month: Option<i32>,
    cover_path: Option<&str>,
    tags: Option<&str>,
    condition: Option<&str>,
    now: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO entries (series_id, issue_number, title, pub_year, pub_month, cover_path, tags, condition, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
        params![series_id, issue_number, title, pub_year, pub_month, cover_path, tags, condition, now],
    )?; 
    Ok(conn.last_insert_rowid())
}
