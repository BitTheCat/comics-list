mod publisher;
mod series;

pub fn handler<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        series::api_list_series_paginated,
        series::api_create_series,
        series::api_update_series,
        series::api_delete_series,
        publisher::api_list_publishers,
        publisher::api_create_publisher
    ]
}
