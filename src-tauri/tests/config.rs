use std::path::{Path, PathBuf};

use comics_list_tauri::config::{
    find_or_create_config, resolve_db_path, save_config, validate_covers_dir, validate_db_path,
    AppConfig,
};
use comics_list_tauri::models::config::UpdateConfigPayload;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("comics-list-config-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn as_str(path: &Path) -> &str {
    path.to_str().expect("utf-8 path")
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
    let db = dir.join("comics-list.db");
    let config = AppConfig {
        db_path: Some(as_str(&db).to_string()),
        theme: Some("dark".to_string()),
        language: Some("it".to_string()),
        ..Default::default()
    };

    save_config(&path, &config).expect("save config");
    let loaded = find_or_create_config(&path).expect("reload config");

    assert!(loaded.portable);
    assert_eq!(loaded.db_path.as_deref(), Some(as_str(&db)));
    assert_eq!(loaded.theme.as_deref(), Some("dark"));
    assert_eq!(loaded.language.as_deref(), Some("it"));
    assert!(loaded.covers_dir.is_none());
}

#[test]
fn save_config_is_atomic_and_leaves_no_temp_file() {
    let dir = temp_dir("atomic");
    let path = dir.join("config.toml");

    save_config(&path, &AppConfig::default()).expect("first save");
    save_config(&path, &AppConfig::default()).expect("overwrite");

    assert!(path.exists());
    assert!(
        !dir.join("config.toml.tmp").exists(),
        "temp file must be renamed away"
    );
}

#[test]
fn apply_update_sets_provided_fields_only() {
    let dir = temp_dir("apply");
    let db = dir.join("comics.db");
    let mut config = AppConfig {
        theme: Some("light".to_string()),
        ..Default::default()
    };

    config
        .apply(UpdateConfigPayload {
            db_path: Some(format!("  {}  ", as_str(&db))),
            language: Some("en".to_string()),
            ..Default::default()
        })
        .expect("apply");

    assert_eq!(config.db_path.as_deref(), Some(as_str(&db)));
    assert_eq!(config.language.as_deref(), Some("en"));
    assert_eq!(config.theme.as_deref(), Some("light"), "untouched field");
    assert!(config.portable, "portable is never changed by an update");
}

#[test]
fn apply_update_clears_value_with_empty_string() {
    let dir = temp_dir("clear");
    let db = dir.join("comics.db");
    let covers = dir.join("covers");
    let mut config = AppConfig {
        db_path: Some(as_str(&db).to_string()),
        covers_dir: Some(as_str(&covers).to_string()),
        ..Default::default()
    };

    config
        .apply(UpdateConfigPayload {
            db_path: Some("   ".to_string()),
            ..Default::default()
        })
        .expect("apply");

    assert!(config.db_path.is_none());
    assert_eq!(config.covers_dir.as_deref(), Some(as_str(&covers)));
}

#[test]
fn rejected_update_changes_nothing() {
    let dir = temp_dir("rejected");
    let db = dir.join("comics.db");
    let mut config = AppConfig {
        theme: Some("light".to_string()),
        ..Default::default()
    };

    // Valid theme + invalid (relative) db path: the whole update is refused.
    let result = config.apply(UpdateConfigPayload {
        theme: Some("dark".to_string()),
        db_path: Some("relative/comics.db".to_string()),
        ..Default::default()
    });

    assert!(result.is_err());
    assert_eq!(config.theme.as_deref(), Some("light"));
    assert!(config.db_path.is_none());

    // A later valid update still works.
    config
        .apply(UpdateConfigPayload {
            db_path: Some(as_str(&db).to_string()),
            ..Default::default()
        })
        .expect("valid update");
    assert_eq!(config.db_path.as_deref(), Some(as_str(&db)));
}

#[test]
fn db_path_accepts_absolute_db_file() {
    let dir = temp_dir("db-ok");

    assert!(validate_db_path(as_str(&dir.join("comics.db"))).is_ok());
    assert!(validate_db_path(as_str(&dir.join("COMICS.DB"))).is_ok());
}

#[test]
fn db_path_rejects_relative_path() {
    assert!(validate_db_path("comics.db").is_err());
    assert!(validate_db_path("data/comics.db").is_err());
}

#[test]
fn db_path_rejects_wrong_extension() {
    let dir = temp_dir("db-ext");

    assert!(validate_db_path(as_str(&dir.join("comics.txt"))).is_err());
    assert!(validate_db_path(as_str(&dir.join("comics"))).is_err());
    assert!(validate_db_path(as_str(&dir.join(".db"))).is_err());
}

#[test]
fn db_path_rejects_directory() {
    let dir = temp_dir("db-dir");
    let folder = dir.join("folder.db");
    std::fs::create_dir_all(&folder).expect("create dir");

    assert!(validate_db_path(as_str(&folder)).is_err());
}

#[test]
fn db_path_rejects_parent_dir_components() {
    let dir = temp_dir("db-parent");
    let sneaky = dir.join("..").join("comics.db");

    assert!(validate_db_path(as_str(&sneaky)).is_err());
}

#[test]
fn paths_reject_network_and_device_paths() {
    for path in [
        r"\\server\share\comics.db",
        "//server/share/comics.db",
        r"\\?\C:\data\comics.db",
        r"\\.\pipe\comics.db",
    ] {
        assert!(validate_db_path(path).is_err(), "{path} must be rejected");
        assert!(
            validate_covers_dir(path).is_err(),
            "{path} must be rejected"
        );
    }
}

#[test]
fn covers_dir_accepts_absolute_existing_or_new_directory() {
    let dir = temp_dir("covers-ok");

    assert!(validate_covers_dir(as_str(&dir)).is_ok(), "existing dir");
    assert!(
        validate_covers_dir(as_str(&dir.join("not-created-yet"))).is_ok(),
        "new dir"
    );
}

#[test]
fn covers_dir_rejects_relative_path_parent_dir_and_files() {
    let dir = temp_dir("covers-bad");
    let file = dir.join("a-file.txt");
    std::fs::write(&file, "x").expect("write file");

    assert!(validate_covers_dir("covers").is_err());
    assert!(validate_covers_dir(as_str(&dir.join("..").join("covers"))).is_err());
    assert!(validate_covers_dir(as_str(&file)).is_err());
}

#[test]
fn corrupt_config_is_backed_up_and_replaced_with_defaults() {
    let dir = temp_dir("corrupt");
    let path = dir.join("config.toml");
    std::fs::write(&path, "portable = [ this is not toml").expect("write config");

    let config = find_or_create_config(&path).expect("must recover, not fail");

    assert!(config.portable);
    let backup = std::fs::read_to_string(dir.join("config.toml.bak")).expect("backup exists");
    assert_eq!(backup, "portable = [ this is not toml");
    // The file on disk is now a valid default config.
    assert!(find_or_create_config(&path).expect("reload").portable);
}

#[test]
fn config_with_invalid_paths_is_backed_up_and_replaced_with_defaults() {
    let dir = temp_dir("invalid-values");
    let path = dir.join("config.toml");
    std::fs::write(
        &path,
        "theme = \"dark\"\ndb_path = \"relative/comics.db\"\n",
    )
    .expect("write config");

    let config = find_or_create_config(&path).expect("must recover, not fail");

    assert!(config.db_path.is_none());
    assert!(config.theme.is_none(), "whole file is replaced by defaults");
    assert!(dir.join("config.toml.bak").exists());
}
