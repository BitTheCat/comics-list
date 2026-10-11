#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(commands::handler())
        .setup(|app| {
            // initialize DB on startup (ignore errors here — commands will report them)
            if let Err(e) = db::init_db(app.handle()) {
                println!("failed to init db: {}", e);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
