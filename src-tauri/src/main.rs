#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::{create_series, init_db, list_series, SeriesRow};
use serde::Serialize;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
fn api_list_series() -> Result<Vec<SeriesRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    list_series(&conn).map_err(|e| format!("db query error: {}", e))
}

#[derive(Serialize)]
struct CreateSeriesPayload {
    pub title: String,
    pub series_title: Option<String>,
    pub publisher_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
}

#[tauri::command]
fn api_create_series(payload: CreateSeriesPayload) -> Result<String, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    create_series(
        &conn,
        &id,
        &payload.title,
        payload.series_title.as_deref(),
        payload.publisher_id.as_deref(),
        payload.start_year,
        payload.end_year,
        payload.notes.as_deref(),
        &now,
    )
    .map_err(|e| format!("db insert error: {}", e))?;
    Ok(id)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api_list_series, api_create_series])
        .setup(|app| {
            // initialize DB on startup (ignore errors here — commands will report them)
            if let Err(e) = init_db() {
                println!("failed to init db: {}", e);
            }
            let _ = app.get_window("main");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

