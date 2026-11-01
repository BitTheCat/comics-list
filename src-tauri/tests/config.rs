use std::path::{Path, PathBuf};

use comics_list_tauri::config::{find_or_create_config, resolve_db_path, save_config, AppConfig};
use comics_list_tauri::models::config::UpdateConfigPayload;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("comics-list-config-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn default_config_is_portable() {
    let config = AppConfig::default();

    assert!(config.portable);
    assert!(config.db_path.is_none());
    assert!(config.covers_dir.is_none());
    assert!(config.theme.is_none());
    assert!(config.language.is_none());
}

#[test]
fn load_or_create_creates_file_with_defaults() {
    let dir = temp_dir("create");
    let path = dir.join("config.toml");

    assert!(!path.exists(), "config should not exist yet");

    let config = find_or_create_config(&path).expect("find_or_create");

    assert!(config.portable);
    assert!(path.exists(), "config file should be created");

    // A second call reads the existing file (no error, same defaults).
    let again = find_or_create_config(&path).expect("find_or_create again");
    assert!(again.portable);
}

#[test]
fn load_or_create_reads_existing_values() {
    let dir = temp_dir("read");
    let path = dir.join("config.toml");
    std::fs::write(&path, "portable = false\n").expect("write config");

    let config = find_or_create_config(&path).expect("find_or_create");

    assert!(!config.portable);
}

#[test]
fn resolve_db_path_prefers_explicit_path() {
    let config = AppConfig {
        db_path: Some("/custom/collection.db".to_string()),
        ..Default::default()
    };

    let path = resolve_db_path(&config, Path::new("/exe"), Path::new("/appdata"));

    assert_eq!(path, PathBuf::from("/custom/collection.db"));
}

#[test]
fn resolve_db_path_portable_uses_exe_dir() {
    let config = AppConfig {
        portable: true,
        ..Default::default()
    };

    let path = resolve_db_path(&config, Path::new("/exe"), Path::new("/appdata"));

    assert_eq!(path, Path::new("/exe").join("comics-list.db"));
}

#[test]
fn resolve_db_path_non_portable_uses_app_data_dir() {
    let config = AppConfig {
        portable: false,
        ..Default::default()
    };

    let path = resolve_db_path(&config, Path::new("/exe"), Path::new("/appdata"));

    assert_eq!(path, Path::new("/appdata").join("comics-list.db"));
}

#[test]
fn partial_config_file_keeps_portable_default() {
    let dir = temp_dir("partial");
    let path = dir.join("config.toml");
    std::fs::write(&path, "theme = \"dark\"\n").expect("write config");

    let config = find_or_create_config(&path).expect("find_or_create");

    assert!(config.portable, "missing `portable` must fall back to true");
    assert_eq!(config.theme.as_deref(), Some("dark"));
}

#[test]
fn save_config_round_trips() {
    let dir = temp_dir("save");
    let path = dir.join("config.toml");
    let config = AppConfig {
        db_path: Some("D:/comics/comics-list.db".to_string()),
        theme: Some("dark".to_string()),
        language: Some("it".to_string()),
        ..Default::default()
    };

    save_config(&path, &config).expect("save config");
    let loaded = find_or_create_config(&path).expect("reload config");

    assert!(loaded.portable);
    assert_eq!(loaded.db_path.as_deref(), Some("D:/comics/comics-list.db"));
    assert_eq!(loaded.theme.as_deref(), Some("dark"));
    assert_eq!(loaded.language.as_deref(), Some("it"));
    assert!(loaded.covers_dir.is_none());
}

#[test]
fn apply_update_sets_provided_fields_only() {
    let mut config = AppConfig {
        theme: Some("light".to_string()),
        ..Default::default()
    };

    config.apply(UpdateConfigPayload {
        db_path: Some("  D:/db/comics.db  ".to_string()),
        language: Some("en".to_string()),
        ..Default::default()
    });

    assert_eq!(config.db_path.as_deref(), Some("D:/db/comics.db"));
    assert_eq!(config.language.as_deref(), Some("en"));
    assert_eq!(config.theme.as_deref(), Some("light"), "untouched field");
    assert!(config.portable, "portable is never changed by an update");
}

#[test]
fn apply_update_clears_value_with_empty_string() {
    let mut config = AppConfig {
        db_path: Some("D:/db/comics.db".to_string()),
        covers_dir: Some("D:/covers".to_string()),
        ..Default::default()
    };

    config.apply(UpdateConfigPayload {
        db_path: Some("   ".to_string()),
        ..Default::default()
    });

    assert!(config.db_path.is_none());
    assert_eq!(config.covers_dir.as_deref(), Some("D:/covers"));
}
