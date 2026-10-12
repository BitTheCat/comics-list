mod entry;
mod publisher;
mod series;

pub fn handler<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        // Series
        series::api_list_series_paginated,
        series::api_create_series,
        series::api_update_series,
        series::api_delete_series,
        // Publisher
        publisher::api_list_publishers,
        publisher::api_create_publisher,
        // Entry
        entry::api_list_entries_paginated,
        entry::api_create_entry,
        entry::api_update_entry,
        entry::api_delete_entry,
    ]
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use rusqlite::Connection;

    #[test]
    fn mutex_connection_persists_across_locks() {
        let state = Mutex::new(Connection::open_in_memory().unwrap());

        // primo lock: crea tabella + inserisci
        {
            let conn = state.lock().unwrap();
            conn.execute_batch("CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT)")
                .unwrap();
            conn.execute("INSERT INTO t (name) VALUES (?1)", ["Spawn"])
                .unwrap();
        } // ← qui il guard esce di scope: il lock si rilascia

        // secondo lock: se il lock NON fosse stato rilasciato, qui andremmo in deadlock
        {
            let conn = state.lock().unwrap();
            let count: i64 = conn
                .query_row("SELECT COUNT(*) FROM t", [], |row| row.get(0))
                .unwrap();
            assert_eq!(count, 1); // i dati sono ancora lì → stessa connessione
        }
    }
}