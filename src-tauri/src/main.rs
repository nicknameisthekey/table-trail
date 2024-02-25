#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents additional console window on Windows in release, DO NOT REMOVE!!

use std::collections::HashMap;

use app::App;
use async_std::task;

mod app;
mod app_config;
mod front_models;
mod postgres;
mod sqlite;

#[tauri::command]
fn tables() -> Vec<String> {
    let items = task::block_on(postgres::tables()).unwrap();
    let r = items.iter().map(|t| t.name.clone()).collect();
    r
}

#[tauri::command(rename_all = "snake_case")]
fn send_query(query: String) -> Vec<HashMap<String, String>> {
    let items = task::block_on(postgres::send_query(query)).unwrap();
    items
}

fn main() {
    let app = task::block_on(App::new());
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            tables,
            send_query,
            app_config::connection_profiles,
            app_config::add_profile,
            app::db_objects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
