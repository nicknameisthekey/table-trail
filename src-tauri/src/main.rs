// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use app::App;
use async_std::task;
use db_config::DatabaseConfig;
use tauri::State;

mod app;
mod db_config;
mod postgres;

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

#[tauri::command(rename_all = "snake_case")]
async fn connection_configs(app: State<'_, App>) -> Result<Vec<DatabaseConfig>, ()> {
    let configs = app.db_configs().await;
    Ok(configs)
}

#[tauri::command(rename_all = "snake_case")]
async fn add_db_config(app: State<'_, App>, config: DatabaseConfig) -> Result<(), ()> {
    app.add_db_config(config).await;
    Ok(())
}

fn main() {
    let app = task::block_on(App::new());
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            tables,
            send_query,
            connection_configs,
            add_db_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
