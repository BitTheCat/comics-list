use chrono::Utc;

use crate::db::{init_db, create_series, update_series, delete_series, list_series_paginated, SeriesRow};
use crate::models::pagination::{PaginationRequest, PaginationResponse};
use crate::models::series::SeriesPayload;

#[tauri::command]
pub fn api_create_series(payload: SeriesPayload) -> Result<i64, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
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
    request: PaginationRequest,
) -> Result<PaginationResponse<SeriesRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;

    let page = request.page.unwrap_or(1).max(1);
    let page_size = request.page_size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * page_size;

    list_series_paginated(&conn, page_size, offset).map_err(|e| format!("db query error: {}", e))
}

#[tauri::command]
pub fn api_update_series(payload: SeriesPayload) -> Result<i64, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
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
pub fn api_delete_series(id: i64) -> Result<(), String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    delete_series(&conn, id).map_err(|e| format!("db delete error: {}", e))?;
    Ok(())
}