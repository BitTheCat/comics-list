use std::sync::Mutex;

use crate::config::{config_path, save_config, AppConfig};
use crate::models::config::UpdateConfigPayload;

#[tauri::command]
pub fn api_get_config(state: tauri::State<'_, Mutex<AppConfig>>) -> Result<AppConfig, String> {
    let config = state.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

#[tauri::command]
pub fn api_update_config(
    state: tauri::State<'_, Mutex<AppConfig>>,
    payload: UpdateConfigPayload,
) -> Result<AppConfig, String> {
    let mut config = state.lock().map_err(|e| e.to_string())?;

    // Update a copy first: the in-memory state changes only if the update is
    // valid and the file is saved.
    let mut updated = config.clone();
    updated.apply(payload)?;
    save_config(&config_path()?, &updated).map_err(|e| format!("config save error: {}", e))?;

    *config = updated;
    Ok(config.clone())
}
