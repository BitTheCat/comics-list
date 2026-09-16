use rusqlite::{params, Connection, Result};
use serde::Serialize;

use crate::models::pagination::{PaginationMeta, PaginationResponse};

#[derive(Debug, Serialize)]
pub struct EntryRow {
    pub id: i64,
    pub series_id: i64,
    pub issue_number: String,
    pub title: Option<String>,
    pub pub_year: Option<i32>,
    pub pub_month: Option<i32>,
    pub cover_path: Option<String>,
    pub tags: Vec<String>,
    pub condition: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_entry(
    conn: &Connection,
    series_id: i64,
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
        "INSERT INTO entries (series_id, issue_number, title, pub_year, pub_month, cover_path, tags, condition, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        params![series_id, issue_number, title, pub_year, pub_month, cover_path, tags, condition, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_entries_paginated(
    conn: &Connection,
    series_id: i64,
    query: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<PaginationResponse<EntryRow>> {
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM entries WHERE series_id = ?1",
        [series_id],
        |row| row.get(0),
    )?;

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
        WHERE e.series_id = ?1
        ORDER BY e.issue_number COLLATE NOCASE
        LIMIT ?2 OFFSET ?3",
    )?;
    let rows = stmt.query_map(params![series_id, limit, offset], |r| {
        let tags_raw: Option<String> = r.get(7).ok().flatten();

        Ok(EntryRow {
            id: r.get(0)?,
            series_id: r.get(1)?,
            issue_number: r.get(2)?,
            title: r.get(3).ok(),
            pub_year: r.get(4).ok(),
            pub_month: r.get(5).ok(),
            cover_path: r.get(6).ok(),
            tags: parse_tags(tags_raw.as_deref()),
            condition: r.get(8).ok(),
            created_at: r.get(9)?,
            updated_at: r.get(10)?,
        })
    })?;

    let mut res = Vec::new();
    for s in rows {
        res.push(s?);
    }
    Ok(PaginationResponse {
        items: res,
        pagination: PaginationMeta {
            page: offset / limit + 1,
            page_size: limit,
            total_items: total,
            total_pages: (total + limit - 1) / limit,
        },
    })
}

pub fn update_entry(
    conn: &Connection,
    id: Option<i64>,
    series_id: i64,
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
        "UPDATE entries SET series_id = ?1, issue_number = ?2, title = ?3, pub_year = ?4, pub_month = ?5, cover_path = ?6, tags = ?7, condition = ?8, updated_at = ?9 WHERE id = ?10",
        params![series_id, issue_number, title, pub_year, pub_month, cover_path, tags, condition, now, id],
    )?;
    Ok(id.unwrap_or(conn.last_insert_rowid()))
}

pub fn delete_entry(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM entries WHERE id = ?1", params![id])?;
    Ok(())
}

fn parse_tags(tags: Option<&str>) -> Vec<String> {
    tags.unwrap_or_default()
        .split(',')
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.to_string())
        .collect()
}
