use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::Manager;

use crate::models::config::UpdateConfigPayload;

// `#[serde(default)]` at struct level fills missing fields from `Default`,
// so a partial file (e.g. only `theme = "dark"`) keeps `portable = true`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub portable: bool,
    pub db_path: Option<String>,
    pub covers_dir: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            portable: true,
            db_path: None,
            covers_dir: None,
            theme: None,
            language: None,
        }
    }
}

impl AppConfig {
    /// Applies a partial update. `portable` is never changed from here.
    pub fn apply(&mut self, update: UpdateConfigPayload) {
        if let Some(value) = update.db_path {
            self.db_path = normalize(value);
        }
        if let Some(value) = update.covers_dir {
            self.covers_dir = normalize(value);
        }
        if let Some(value) = update.theme {
            self.theme = normalize(value);
        }
        if let Some(value) = update.language {
            self.language = normalize(value);
        }
    }
}

fn normalize(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Returns the path to the config file: near the executable.
pub fn config_path() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let dir = exe
        .parent()
        .ok_or("failed to get parent directory of executable")?;
    Ok(dir.join("config.toml"))
}

pub fn load_or_create() -> Result<AppConfig, String> {
    find_or_create_config(&config_path()?)
}

pub fn find_or_create_config(path: &Path) -> Result<AppConfig, String> {
    if path.exists() {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        toml::from_str(&content).map_err(|e| e.to_string())
    } else {
        let config = AppConfig::default();
        save_config(path, &config)?;
        Ok(config)
    }
}

pub fn save_config(path: &Path, config: &AppConfig) -> Result<(), String> {
    let content = toml::to_string(config).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
}

pub fn resolve_db_path(config: &AppConfig, exe_dir: &Path, app_dir: &Path) -> PathBuf {
    let db_name = "comics-list.db";
    if let Some(path) = &config.db_path {
        return PathBuf::from(path);
    }
    if config.portable {
        exe_dir.join(db_name)
    } else {
        app_dir.join(db_name)
    }
}

pub fn db_path(config: &AppConfig, app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("failed to get parent directory of executable")?
        .to_path_buf();
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .to_path_buf();
    Ok(resolve_db_path(config, &exe_dir, &app_dir))
}
