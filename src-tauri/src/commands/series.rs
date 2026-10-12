use chrono::Utc;
use std::sync::Mutex;
use rusqlite::Connection;

use crate::db::{
    create_series, delete_series, list_series_paginated, update_series, SeriesRow,
};
use crate::models::pagination::{PaginationRequest, PaginationResponse};
use crate::models::series::{CreateSeriesPayload, UpdateSeriesPayload};

#[tauri::command]
pub fn api_create_series(
    state: tauri::State<'_, Mutex<Connection>>,
    payload: CreateSeriesPayload,
) -> Result<i64, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let id = create_series(
        &conn,
        &payload.title,
        payload.description.as_deref(),
        payload.publisher_id,
        payload.start_year,
        payload.end_year,
        payload.notes.as_deref(),
        &now,
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_list_series_paginated(
    state: tauri::State<'_, Mutex<Connection>>,
    request: PaginationRequest,
) -> Result<PaginationResponse<SeriesRow>, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;

    let page = request.page.unwrap_or(1).max(1);
    let page_size = request.page_size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * page_size;
    let query = request.query.as_deref();

    list_series_paginated(&conn, query, page_size, offset)
        .map_err(|e| format!("db query error: {}", e))
}

#[tauri::command]
pub fn api_update_series(
    state: tauri::State<'_, Mutex<Connection>>,
    payload: UpdateSeriesPayload,
) -> Result<i64, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let id = update_series(
        &conn,
        payload.id,
        &payload.title,
        payload.description.as_deref(),
        payload.publisher_id,
        payload.start_year,
        payload.end_year,
        payload.notes.as_deref(),
        &now,
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_delete_series(
    state: tauri::State<'_, Mutex<Connection>>,
    id: i64
) -> Result<(), String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    delete_series(&conn, id).map_err(|e| format!("db delete error: {}", e))?;
    Ok(())
}
