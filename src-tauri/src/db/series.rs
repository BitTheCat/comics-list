use rusqlite::{params, Connection, Result};
use serde::Serialize;

use crate::models::pagination::{PaginationMeta, PaginationResponse};

use super::publisher::PublisherRow;

#[derive(Debug, Serialize)]
pub struct SeriesRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub publisher: Option<PublisherRow>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub entries_count: i64,
}

pub fn list_series_paginated(
    conn: &Connection,
    limit: i64,
    offset: i64,
) -> Result<PaginationResponse<SeriesRow>> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM series", [], |row| row.get(0))?;

    let mut stmt = conn.prepare(
        "SELECT
            s.id,
            s.title,
            s.description,
            p.id AS publisher_id,
            p.name AS publisher_name,
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
        LEFT JOIN publishers p ON p.id = s.publisher_id
        ORDER BY s.title COLLATE NOCASE
        LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt.query_map(params![limit, offset], |r| {
        let publisher_id: Option<i64> = r.get(3)?;
        let publisher_name: Option<String> = r.get(4)?;

        let publisher = match (publisher_id, publisher_name) {
            (Some(id), Some(name)) => Some(PublisherRow { id, name }),
            _ => None,
        };

        Ok(SeriesRow {
            id: r.get(0)?,
            title: r.get(1)?,
            description: r.get(2)?,
            publisher,
            start_year: r.get(5)?,
            end_year: r.get(6)?,
            notes: r.get(7)?,
            created_at: r.get(8)?,
            updated_at: r.get(9)?,
            entries_count: r.get(10)?,
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

pub fn create_series(
    conn: &Connection,
    title: &str,
    description: Option<&str>,
    publisher_id: Option<i64>,
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
