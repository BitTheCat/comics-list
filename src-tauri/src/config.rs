use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
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
    /// Paths are validated; if any field is rejected nothing is changed.
    pub fn apply(&mut self, update: UpdateConfigPayload) -> Result<(), String> {
        let mut next = self.clone();

        if let Some(value) = update.db_path {
            let value = normalize(value);
            if let Some(path) = &value {
                validate_db_path(path)?;
            }
            next.db_path = value;
        }
        if let Some(value) = update.covers_dir {
            let value = normalize(value);
            if let Some(path) = &value {
                validate_covers_dir(path)?;
            }
            next.covers_dir = value;
        }
        if let Some(value) = update.theme {
            next.theme = normalize(value);
        }
        if let Some(value) = update.language {
            next.language = normalize(value);
        }

        *self = next;
        Ok(())
    }

    /// Validates the paths of a config read from disk (it may be hand-edited).
    pub fn validate(&self) -> Result<(), String> {
        if let Some(path) = &self.db_path {
            validate_db_path(path)?;
        }
        if let Some(path) = &self.covers_dir {
            validate_covers_dir(path)?;
        }
        Ok(())
    }
}

/// Common checks for user-provided paths: local, absolute, no `..`.
/// Network/device paths (`\\server\share`, `\\?\`) are rejected: opening them
/// on Windows can leak credentials to a remote host.
fn check_path(value: &str) -> Result<&Path, String> {
    if value.starts_with("\\\\") || value.starts_with("//") {
        return Err("network and device paths are not allowed".to_string());
    }
    let path = Path::new(value);
    if !path.is_absolute() {
        return Err("path must be absolute".to_string());
    }
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("path must not contain '..'".to_string());
    }
    Ok(path)
}

pub fn validate_db_path(value: &str) -> Result<(), String> {
    let path = check_path(value)?;
    let has_db_extension = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("db"));
    if !has_db_extension {
        return Err("database path must be a file ending with .db".to_string());
    }
    if path.is_dir() {
        return Err("database path points to a directory".to_string());
    }
    Ok(())
}

pub fn validate_covers_dir(value: &str) -> Result<(), String> {
    let path = check_path(value)?;
    if path.exists() && !path.is_dir() {
        return Err("covers path exists and is not a directory".to_string());
    }
    Ok(())
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

/// Reads the config, creating it with defaults when missing.
/// A file that is corrupt or has invalid values (it may be hand-edited) is not
/// fatal: it is moved to `config.toml.bak` and replaced with the defaults.
pub fn find_or_create_config(path: &Path) -> Result<AppConfig, String> {
    if !path.exists() {
        let config = AppConfig::default();
        save_config(path, &config)?;
        return Ok(config);
    }

    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let parsed = String::from_utf8(bytes)
        .map_err(|e| e.to_string())
        .and_then(|content| toml::from_str::<AppConfig>(&content).map_err(|e| e.to_string()))
        .and_then(|config| config.validate().map(|_| config));

    match parsed {
        Ok(config) => Ok(config),
        Err(reason) => {
            let backup = backup_path(path);
            eprintln!(
                "invalid config {}: {}; saved as {} and using defaults",
                path.display(),
                reason,
                backup.display()
            );
            std::fs::rename(path, &backup).map_err(|e| e.to_string())?;
            let config = AppConfig::default();
            save_config(path, &config)?;
            Ok(config)
        }
    }
}

fn backup_path(path: &Path) -> PathBuf {
    path.with_extension("toml.bak")
}

/// Saves atomically (write a temp file, then rename) so an interrupted write
/// cannot leave a truncated config behind.
pub fn save_config(path: &Path, config: &AppConfig) -> Result<(), String> {
    let content = toml::to_string(config).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("toml.tmp");
    std::fs::write(&tmp, content).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        e.to_string()
    })
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
