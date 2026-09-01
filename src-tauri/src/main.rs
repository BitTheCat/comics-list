#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::{create_work, init_db, list_works, WorkRow};
use serde::Serialize;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
fn api_list_works() -> Result<Vec<WorkRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    list_works(&conn).map_err(|e| format!("db query error: {}", e))
}

#[derive(Serialize)]
struct CreateWorkPayload {
    pub title: String,
    pub description: Option<String>,
}

#[tauri::command]
fn api_create_work(payload: CreateWorkPayload) -> Result<String, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    create_work(&conn, &id, &payload.title, payload.description.as_deref(), &now)
        .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api_list_works, api_create_work])
        .setup(|app| {
            // initialize DB on startup (ignore errors here — commands will report them)
            if let Err(e) = init_db() {
                println!(\"failed to init db: {}\", e);
            }
            let _ = app.get_window(\"main\");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect(\"error while running tauri application\");
}

