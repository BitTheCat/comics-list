#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use chrono::Utc;
use db::{create_series, init_db, list_series, SeriesRow};
use serde::Deserialize;
use tauri::Manager;

#[tauri::command]
fn api_list_series() -> Result<Vec<SeriesRow>, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    list_series(&conn).map_err(|e| format!("db query error: {}", e))
}

#[derive(Deserialize)]
struct CreateSeriesPayload {
    pub title: String,
    pub description: Option<String>,
    pub publisher_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
}

#[tauri::command]
fn api_create_series(payload: CreateSeriesPayload) -> Result<i64, String> {
    let conn = init_db().map_err(|e| format!("db init error: {}", e))?;
    let now = Utc::now().to_rfc3339();
    let id = create_series(
        &conn,
        &payload.title,
        payload.description.as_deref(),
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
