#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;

use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(commands::handler())
        .setup(|app| {
            let conn = db::init_db(app.handle())
                .expect("failed to initialize db");
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
