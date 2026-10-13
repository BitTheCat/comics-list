use chrono::Utc;
use rusqlite::Connection;
use std::sync::Mutex;

use crate::db::{
    create_entry, delete_entry, list_entries_paginated, next_issue_number, update_entry, EntryRow,
};
use crate::models::entry::{CreateEntryPayload, ListEntriesRequest, UpdateEntryPayload};
use crate::models::pagination::PaginationResponse;

#[tauri::command]
pub fn api_create_entry(
    state: tauri::State<'_, Mutex<Connection>>, 
    payload: CreateEntryPayload
) -> Result<i64, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let tags = payload.tags.as_ref().map(|v| v.join(","));

    let id = create_entry(
        &conn,
        payload.series_id,
        payload.issue_number.as_str(),
        payload.title.as_deref(),
        payload.pub_year,
        payload.pub_month,
        payload.cover_path.as_deref(),
        tags.as_deref(),
        payload.condition.as_deref(),
        &now,
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_list_entries_paginated(
    state: tauri::State<'_, Mutex<Connection>>, 
    request: ListEntriesRequest,
) -> Result<PaginationResponse<EntryRow>, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;

    let series_id = request.series_id;
    let query = request.pagination.query.as_deref();
    let page = request.pagination.page.unwrap_or(1).max(1);
    let page_size = request.pagination.page_size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * page_size;

    list_entries_paginated(&conn, series_id, query, page_size, offset)
        .map_err(|e| format!("db query error: {}", e))
}

#[tauri::command]
pub fn api_update_entry(
    state: tauri::State<'_, Mutex<Connection>>, 
    payload: UpdateEntryPayload
) -> Result<i64, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let tags = payload.tags.as_ref().map(|v| v.join(","));

    let id = update_entry(
        &conn,
        payload.id,
        payload.series_id,
        payload.issue_number.as_str(),
        payload.title.as_deref(),
        payload.pub_year,
        payload.pub_month,
        payload.cover_path.as_deref(),
        tags.as_deref(),
        payload.condition.as_deref(),
        &now,
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_delete_entry(
    state: tauri::State<'_, Mutex<Connection>>, 
    id: i64
) -> Result<(), String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    delete_entry(&conn, id).map_err(|e| format!("db delete error: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn api_retrieve_next_issue_number(
    state: tauri::State<'_, Mutex<Connection>>,
    series_id: i64,
) -> Result<String, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    next_issue_number(&conn, series_id).map_err(|e| format!("db query error: {}", e))
}
