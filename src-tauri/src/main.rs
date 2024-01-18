// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::task;

mod postgres;

#[tauri::command]
fn tables() -> Vec<String> {
  let items = task::block_on(postgres::tables()).unwrap();
  let r = items.iter().map(|t|t.name.clone()).collect();
  r
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tables])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
