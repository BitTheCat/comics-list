use rusqlite::Connection;
use std::path::Path;

use super::migrations::run_migrations;

pub fn init_db(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("could not create app data directory: {}", e))?;
    }

    let conn = Connection::open(&path)
        .map_err(|e| format!("could not open database at {}: {}", path.display(), e))?;

    run_migrations(&conn).map_err(|e| format!("could not apply migrations: {}", e))?;
    Ok(conn)
}
