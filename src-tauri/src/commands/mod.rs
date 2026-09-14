mod publisher;
mod series;
mod entry;

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
