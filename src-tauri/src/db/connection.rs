use rusqlite::{Connection, Result};
use std::path::PathBuf;

use super::migrations::run_migrations;

pub fn db_path() -> Option<PathBuf> {
    std::env::current_dir().ok().map(|mut p| {
        p.push("comics-list.db");
        p
    })
}

pub fn init_db() -> Result<Connection> {
    let path = db_path().expect("could not determine current working directory");
    let conn = Connection::open(path)?;
    run_migrations(&conn)?;
    Ok(conn)
}
