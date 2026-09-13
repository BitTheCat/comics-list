use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublisherRow {
    pub id: i64,
    pub name: String,
}

pub fn create_publisher(
    conn: &Connection,
    name: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO publishers (name) VALUES (?1)",
        params![name],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_publishers(conn: &Connection) -> Result<Vec<PublisherRow>> {
    let mut stmt = conn.prepare(
        "SELECT
            p.id,
            p.name
        FROM publishers p
        ORDER BY p.name COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(PublisherRow {
            id: r.get(0)?,
            name: r.get(1)?,
        })
    })?;

    let mut res = Vec::new();
    for s in rows {
        res.push(s?);
    }
    Ok(res)
}
