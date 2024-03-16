#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents additional console window on Windows in release, DO NOT REMOVE!!

use app::App;
use async_std::task;

mod app;
mod app_config;
mod front_models;
mod postgres;
mod sqlite;

fn main() {
    let app = task::block_on(App::new());
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            app::send_query,
            app_config::connection_profiles,
            app_config::add_profile,
            app_config::delete_profile,
            app::db_objects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
