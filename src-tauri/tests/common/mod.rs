use std::sync::{Mutex, OnceLock};

use rusqlite::Connection;

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

fn init_db_once() -> Mutex<Connection> {
    let conn = Connection::open_in_memory().expect("failed to open in-memory db");

    conn.execute_batch(include_str!("../../migrations/0001_initial_schema.sql"))
        .expect("failed to apply migration 0001");
    conn.execute_batch(include_str!(
        "../../migrations/0002_rename_series_title_to_description.sql"
    ))
    .expect("failed to apply migration 0002");
    conn.execute_batch(include_str!("../../migrations/0003_integer_series_ids.sql"))
        .expect("failed to apply migration 0003");
    conn.execute_batch(include_str!(
        "../../migrations/0004_add_entry_condition.sql"
    ))
    .expect("failed to apply migration 0004");
    conn.execute_batch(include_str!(
        "../../migrations/0005_remove_old_column_entry.sql"
    ))
    .expect("failed to apply migration 0005");
    conn.execute_batch(include_str!(
        "../../migrations/0006_remove_notes_column_publisher.sql"
    ))
    .expect("failed to apply migration 0006");
    conn.execute_batch(include_str!(
        "../../migrations/0007_integer_publishers_ids.sql"
    ))
    .expect("failed to apply migration 0007");

    Mutex::new(conn)
}

fn reset_data(conn: &Connection) {
    conn.execute("DELETE FROM entries", [])
        .expect("clear entries");
    conn.execute("DELETE FROM series", [])
        .expect("clear series");
    conn.execute("DELETE FROM publishers", [])
        .expect("clear publishers");
}

pub fn with_test_db<T>(test_fn: impl FnOnce(&Connection) -> T) -> T {
    let db = DB.get_or_init(init_db_once);
    let conn = db.lock().expect("failed to lock test db");

    reset_data(&conn);
    test_fn(&conn)
}
