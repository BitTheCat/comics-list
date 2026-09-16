use crate::db::{create_publisher, init_db, list_publishers, PublisherRow};
use crate::models::publisher::CreatePublisherPayload;

#[tauri::command]
pub fn api_create_publisher(payload: CreatePublisherPayload) -> Result<i64, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;

    let id = create_publisher(&conn, &payload.name).map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

#[tauri::command]
pub fn api_list_publishers(
    name: Option<String>,
) -> Result<Vec<PublisherRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    list_publishers(&conn, name.as_deref()).map_err(|e| format!("db query error: {}", e))
}
