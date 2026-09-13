
use crate::models::publisher::CreatePublisherPayload;
use crate::db::{init_db, create_publisher, list_publishers, PublisherRow};


#[tauri::command]
pub fn api_create_publisher(payload: CreatePublisherPayload) -> Result<i64, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;

    let id = create_publisher(
        &conn,
        &payload.name
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}


#[tauri::command]
pub fn api_list_publishers() -> Result<Vec<PublisherRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    list_publishers(&conn).map_err(|e| format!("db query error: {}", e))
}
