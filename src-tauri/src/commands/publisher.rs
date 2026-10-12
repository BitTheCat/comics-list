use std::sync::Mutex;
use rusqlite::Connection;

use crate::db::{create_publisher, list_publishers, PublisherRow};
use crate::models::publisher::CreatePublisherPayload;

#[tauri::command]
pub fn api_create_publisher(
    state: tauri::State<'_, Mutex<Connection>>,
    payload: CreatePublisherPayload,
) -> Result<i64, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;

    let id =
        create_publisher(&conn, &payload.name).map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_list_publishers(
    state: tauri::State<'_, Mutex<Connection>>,
    name: Option<String>,
) -> Result<Vec<PublisherRow>, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    list_publishers(&conn, name.as_deref()).map_err(|e| format!("db query error: {}", e))
}
