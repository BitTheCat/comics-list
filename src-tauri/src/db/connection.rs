use rusqlite::Connection;
use std::path::PathBuf;

use tauri::Manager;

use super::migrations::run_migrations;

pub fn app_db_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("could not determine app data directory: {}", e))?;

    Ok(app_data_dir.join("comics-list.db"))
}

pub fn init_db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let path = app_db_path(app)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("could not create app data directory: {}", e))?;
    }

    let conn = Connection::open(&path)
        .map_err(|e| format!("could not open database at {}: {}", path.display(), e))?;

    run_migrations(&conn).map_err(|e| format!("could not apply migrations: {}", e))?;

    Ok(conn)
}
