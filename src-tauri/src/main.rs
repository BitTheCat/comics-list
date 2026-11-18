#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod db;
mod models;

use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(commands::handler())
        .setup(|app| {
            // The config sits next to the exe, which may be read-only: keep going
            // with the defaults instead of failing to start.
            let config = config::load_or_create().unwrap_or_else(|e| {
                eprintln!("failed to load config, using defaults: {}", e);
                config::AppConfig::default()
            });
            let path = config::db_path(&config, app.handle()).expect("failed to get db path");
            let conn = db::init_db(&path).expect("failed to initialize db");
            app.manage(Mutex::new(conn));
            app.manage(Mutex::new(config));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
